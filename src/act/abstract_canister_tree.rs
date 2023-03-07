use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        candid_types::CandidTypes,
        canister_methods::CanisterMethods,
        node::{
            candid::func, declaration::Declare, CandidType, CanisterMethod, ExternalCanister,
            GuardFunction, Node, NodeContext,
        },
    },
    generators::{candid_file_generation, random, vm_value_conversion},
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
    pub try_from_vm_value_impls: TokenStream,
    pub try_into_vm_value_impls: TokenStream,
    pub keywords: Vec<String>,
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let canister_declaration_code = self.create_act_not_function_code();

        let child_declarations: Vec<_> =
            self.collect_children()
                .iter()
                .fold(vec![], |acc, child_node| {
                    let child_declarations = child_node.flatten(
                        &NodeContext {
                            cdk_name: self.cdk_name.clone(),
                            keyword_list: self.keywords.clone(),
                        },
                        "Canister".to_string(),
                    );
                    vec![acc, child_declarations].concat()
                });

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code(&self.cdk_name);

        quote! {
            #canister_declaration_code
            #(#child_declarations)*
            #candid_file_generation_code
        }
    }

    fn create_act_not_function_code(&self) -> TokenStream {
        let body = &self.body;
        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation(&self.cdk_name);

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.try_from_vm_value_impls;

        let func_arg_token = func::generate_func_arg_token();

        quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #func_arg_token

            #body
        }
    }

    fn collect_children(&self) -> Vec<Node> {
        let canister_methods: Vec<_> = self
            .collect_canister_methods()
            .iter()
            .map(|canister_method| Node::CanisterMethod(canister_method.clone()))
            .collect();

        let candid_types = self
            .collect_candid_types()
            .iter()
            .map(|candid_type| Node::CandidType(candid_type.clone()))
            .collect();

        let guard_functions = self
            .guard_functions
            .iter()
            .map(|function_guard| Node::GuardFunction(function_guard.clone()))
            .collect();

        let external_canisters = self
            .external_canisters
            .iter()
            .map(|external_canister| Node::ExternalCanister(external_canister.clone()))
            .collect();

        vec![
            canister_methods,
            candid_types,
            guard_functions,
            external_canisters,
        ]
        .concat()
    }

    fn collect_canister_methods(&self) -> Vec<CanisterMethod> {
        let init_method = Some(CanisterMethod::Init(
            self.canister_methods.init_method.clone(),
        ));
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
        let post_upgrade_method = Some(CanisterMethod::PostUpgrade(
            self.canister_methods.post_upgrade_method.clone(),
        ));
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
