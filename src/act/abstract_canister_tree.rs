use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{
    candid_file_generation, random, vm_value_conversion, CandidTypes, CanisterMethods,
    VmValueConversion,
    {
        node::{
            candid::func, AsNode, CandidType, CanisterMethod, Context, ExternalCanister,
            GuardFunction,
        },
        Declaration, Declare,
    },
};

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub candid_types: CandidTypes,
    pub external_canisters: Vec<ExternalCanister>,
    pub guard_functions: Vec<GuardFunction>,
    pub header: TokenStream,
    pub body: TokenStream,
    pub vm_value_conversion: VmValueConversion,
    pub keywords: Vec<String>,
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation();

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.vm_value_conversion.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.vm_value_conversion.try_from_vm_value_impls;

        let func_arg_token = func::generate_func_arg_token();

        let body = &self.body;

        let canister_method_decls = self.generate_declarations(self.collect_canister_methods());
        let candid_type_decls = self.generate_declarations(self.collect_candid_types());
        let guard_function_decls = self.generate_declarations(self.guard_functions.clone());
        let external_canister_decls = self.generate_declarations(self.external_canisters.clone());

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code(&self.cdk_name);

        quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #func_arg_token

            #body

            #(#canister_method_decls)*
            #(#candid_type_decls)*
            #(#guard_function_decls)*
            #(#external_canister_decls)*

            #candid_file_generation_code
        }
    }

    fn generate_declarations<T: AsNode>(&self, list: Vec<T>) -> Vec<Declaration> {
        list.into_iter().fold(vec![], |acc, node| {
            vec![
                acc,
                node.as_node().flatten(
                    &Context {
                        keyword_list: self.keywords.clone(),
                        cdk_name: self.cdk_name.clone(),
                    },
                    "".to_string(),
                ),
            ]
            .concat()
        })
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

        vec![funcs, records, tuples, type_aliases, variants].concat()
    }
}
