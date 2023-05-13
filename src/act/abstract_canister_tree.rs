use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        candid_file_generation, float32, float64, random, vm_value_conversion, CandidTypes,
        CanisterMethods, VmValueConversion,
        {
            node::{AsNode, CandidType, CanisterMethod, Context, GuardFunction},
            Declaration, Declare,
        },
    },
    traits::{HasDefinedNames, HasTypeRefs},
};

use super::node::candid::TypeRef;

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub candid_types: CandidTypes,
    pub guard_functions: Vec<GuardFunction>,
    pub header: TokenStream,
    pub body: TokenStream,
    pub vm_value_conversion: VmValueConversion,
    pub keywords: Vec<String>,
}

pub enum Error {
    TypeNotFound(String),
    GuardFunctionNotFound(String),
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> Result<TokenStream, Vec<Error>> {
        let errors = self
            .verify_type_refs_have_corresponding_definitions()
            .err()
            .into_iter()
            .chain(
                self.verify_guard_function_names_have_corresponding_definitions()
                    .err()
                    .into_iter(),
            )
            .flatten()
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            return Err(errors);
        }

        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation();

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.vm_value_conversion.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.vm_value_conversion.try_from_vm_value_impls;

        let body = &self.body;

        let canister_method_decls = self.generate_declarations(self.collect_canister_methods());
        let candid_type_decls = self.generate_declarations(self.collect_candid_types());
        let guard_function_decls = self.generate_declarations(self.guard_functions.clone());

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code();

        let azle_float64 = float64::generate();
        let azle_float32 = float32::generate();

        Ok(quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #body

            #(#canister_method_decls)*
            #(#candid_type_decls)*
            #(#guard_function_decls)*

            #candid_file_generation_code

            #azle_float64
            #azle_float32
        })
    }

    fn generate_declarations<T: AsNode>(&self, list: Vec<T>) -> Vec<Declaration> {
        list.into_iter().fold(vec![], |acc, node| {
            vec![
                acc,
                node.as_node()
                    .flatten(&self.build_context(), "".to_string()),
            ]
            .concat()
        })
    }

    fn build_context(&self) -> Context {
        Context {
            keyword_list: self.keywords.clone(),
            cdk_name: self.cdk_name.clone(),
        }
    }

    fn collect_canister_methods(&self) -> Vec<CanisterMethod> {
        let init_method = match &self.canister_methods.init_method {
            Some(init_method) => Some(CanisterMethod::Init(init_method.clone())),
            None => None,
        };
        let heartbeat_method = match &self.canister_methods.heartbeat_method {
            Some(heartbeat_method) => Some(CanisterMethod::Heartbeat(heartbeat_method.clone())),
            None => None,
        };
        let inspect_message_method = match &self.canister_methods.inspect_message_method {
            Some(inspect_message_method) => Some(CanisterMethod::InspectMessage(
                inspect_message_method.clone(),
            )),
            None => None,
        };
        let pre_upgrade_method = match &self.canister_methods.pre_upgrade_method {
            Some(pre_upgrade_method) => {
                Some(CanisterMethod::PreUpgrade(pre_upgrade_method.clone()))
            }
            None => None,
        };
        let post_upgrade_method = match &self.canister_methods.post_upgrade_method {
            Some(post_upgrade_method) => {
                Some(CanisterMethod::PostUpgrade(post_upgrade_method.clone()))
            }
            None => None,
        };
        let system_canister_methods: Vec<_> = vec![
            init_method,
            heartbeat_method,
            inspect_message_method,
            pre_upgrade_method,
            post_upgrade_method,
        ]
        .iter()
        .filter_map(|system_canister_method| system_canister_method.clone())
        .collect();

        let query_methods = self
            .canister_methods
            .query_methods
            .iter()
            .map(|query| CanisterMethod::Query(query.clone()))
            .collect();
        let update_methods = self
            .canister_methods
            .update_methods
            .iter()
            .map(|update| CanisterMethod::Update(update.clone()))
            .collect();

        vec![system_canister_methods, query_methods, update_methods].concat()
    }

    fn collect_candid_types(&self) -> Vec<CandidType> {
        let funcs: Vec<_> = self
            .candid_types
            .funcs
            .iter()
            .map(|func| CandidType::Func(func.clone()))
            .collect();
        let records = self
            .candid_types
            .records
            .iter()
            .map(|record| CandidType::Record(record.clone()))
            .collect();
        let services = self
            .candid_types
            .services
            .iter()
            .map(|service| CandidType::Service(service.clone()))
            .collect();
        let tuples = self
            .candid_types
            .tuples
            .iter()
            .map(|tuple| CandidType::Tuple(tuple.clone()))
            .collect();
        let type_aliases = self
            .candid_types
            .type_aliases
            .iter()
            .map(|type_alias| CandidType::TypeAlias(type_alias.clone()))
            .collect();
        let variants = self
            .candid_types
            .variants
            .iter()
            .map(|variant| CandidType::Variant(variant.clone()))
            .collect();

        vec![funcs, records, services, tuples, type_aliases, variants].concat()
    }

    fn verify_type_refs_have_corresponding_definitions(&self) -> Result<(), Vec<Error>> {
        let defined_names: HashSet<_> = self.candid_types.get_defined_names().into_iter().collect();
        let used_names: HashSet<_> = self
            .get_type_refs()
            .iter()
            .map(|type_ref| type_ref.name.clone())
            .collect();

        let diff: Vec<_> = used_names.difference(&defined_names).cloned().collect();

        match diff.is_empty() {
            true => Ok(()),
            false => Err(diff
                .iter()
                .map(|type_ref| Error::TypeNotFound(type_ref.clone()))
                .collect()),
        }
    }

    fn verify_guard_function_names_have_corresponding_definitions(&self) -> Result<(), Vec<Error>> {
        let defined_names_set: HashSet<_> = self
            .guard_functions
            .iter()
            .map(|f| f.name.clone())
            .collect();
        let used_guard_functions: HashSet<_> = self
            .canister_methods
            .collect_used_guard_function_names()
            .into_iter()
            .collect();

        let diff: Vec<_> = used_guard_functions
            .difference(&defined_names_set)
            .cloned()
            .collect();

        match diff.is_empty() {
            true => Ok(()),
            false => Err(diff
                .iter()
                .map(|type_ref| Error::GuardFunctionNotFound(type_ref.clone()))
                .collect()),
        }
    }
}

impl HasTypeRefs for AbstractCanisterTree {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.canister_methods
            .get_type_refs()
            .into_iter()
            .chain(self.candid_types.get_type_refs())
            .collect()
    }
}
