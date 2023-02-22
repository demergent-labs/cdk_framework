use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{
        canister_method::CanisterMethod, data_type::func, proclamation::Proclaim, DataType,
        Declaration, Node, NodeContext,
    },
    generators::{candid_file_generation, random, vm_value_conversion},
    AbstractCanisterTree,
};

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let canister_declaration_code = self.create_act_not_function_code();
        let child_declarations = self.create_child_declarations();

        quote! {
            #canister_declaration_code
            #(#child_declarations)*
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

            #candid_file_generation_code
        }
    }

    fn create_child_declarations(&self) -> Vec<Declaration> {
        self.collect_children()
            .iter()
            .map(|child_node| {
                child_node.create_proclamation(
                    &NodeContext {
                        cdk_name: self.cdk_name.clone(),
                        keyword_list: self.keywords.clone(),
                    },
                    "Canister".to_string(),
                )
            })
            .fold(vec![], |acc, child_proclamation| {
                vec![acc, child_proclamation.flatten()].concat()
            })
    }

    fn collect_children(&self) -> Vec<Node> {
        let canister_methods: Vec<_> = self
            .collect_canister_methods()
            .iter()
            .map(|canister_method| Node::CanisterMethod(canister_method.clone()))
            .collect();

        let data_types = self
            .collect_data_types()
            .iter()
            .map(|data_type| Node::DataType(data_type.clone()))
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
            data_types,
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

    fn collect_data_types(&self) -> Vec<DataType> {
        let funcs: Vec<_> = self
            .data_types
            .funcs
            .iter()
            .map(|func| DataType::Func(func.clone()))
            .collect();
        let records = self
            .data_types
            .records
            .iter()
            .map(|record| DataType::Record(record.clone()))
            .collect();
        let tuples = self
            .data_types
            .tuples
            .iter()
            .map(|tuple| DataType::Tuple(tuple.clone()))
            .collect();
        let type_aliases = self
            .data_types
            .type_aliases
            .iter()
            .map(|type_alias| DataType::TypeAlias(type_alias.clone()))
            .collect();
        let variants = self
            .data_types
            .variants
            .iter()
            .map(|variant| DataType::Variant(variant.clone()))
            .collect();

        vec![funcs, records, tuples, type_aliases, variants].concat()
    }
}
