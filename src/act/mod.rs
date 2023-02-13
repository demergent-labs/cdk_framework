use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::hash::Hash;

use self::{
    node::{
        canister_method::{
            CanisterMethod, CanisterMethodContext,
            {
                HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod,
                PreUpgradeMethod, QueryMethod, UpdateMethod,
            },
        },
        data_type::func,
        {ExternalCanister, FunctionGuard},
    },
    proclamation::{Proclaim, Proclamation},
};
use crate::generators::{candid_file_generation, random, vm_value_conversion};

pub mod node;
pub mod proclamation;
pub mod to_node;

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub external_canisters: Vec<ExternalCanister>, // TODO Make sure these are collected as children
    pub function_guards: Vec<FunctionGuard>,
    pub header: TokenStream,
    pub body: TokenStream,
    pub try_from_vm_value_impls: TokenStream,
    pub try_into_vm_value_impls: TokenStream,
    pub keywords: Vec<String>,
}

pub struct CanisterMethods {
    pub heartbeat_method: Option<HeartbeatMethod>,
    pub init_method: InitMethod,
    pub inspect_message_method: Option<InspectMessageMethod>,
    pub post_upgrade_method: PostUpgradeMethod,
    pub pre_upgrade_method: Option<PreUpgradeMethod>,
    pub query_methods: Vec<QueryMethod>,
    pub update_methods: Vec<UpdateMethod>,
}

impl Proclaim<()> for AbstractCanisterTree {
    fn create_declaration(&self, _: &(), _: String) -> Option<TokenStream> {
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

        Some(quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #func_arg_token

            #body

            #candid_file_generation_code
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("Canister".to_string())
    }

    fn create_inline_declarations(&self, _: &(), _: String) -> HashMap<String, TokenStream> {
        let canister_method_context = CanisterMethodContext {
            cdk_name: self.cdk_name.clone(),
            keyword_list: self.keywords.clone(),
        };

        let canister_methods = self.collect_children();
        let children_declaration = canister_methods
            .create_proclamation(&canister_method_context, "CanisterMethods".to_string());

        flatten_declaration(children_declaration, HashMap::new())
    }
}

fn flatten_declaration(
    declaration: Proclamation,
    map: HashMap<String, TokenStream>,
) -> HashMap<String, TokenStream> {
    let mut result = HashMap::new();
    result.extend(map);
    if let Some(identifier) = declaration.identifier {
        if let Some(code) = declaration.declaration {
            result.insert(identifier, code);
        }
    }
    result.extend(declaration.inline_declarations);
    result
}

fn combine_maps<K, V>(map1: HashMap<K, V>, map2: HashMap<K, V>) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut result = HashMap::new();

    result.extend(map1);
    result.extend(map2);

    result
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> TokenStream {
        let canister_declaration = self.create_proclamation(&(), "Canister".to_string());

        let canister_declaration_code = match canister_declaration.declaration {
            Some(code) => code,
            None => quote!(),
        };

        let canister_declaration_children: Vec<_> = canister_declaration
            .inline_declarations
            .values()
            .cloned()
            .collect();

        quote! {
            #canister_declaration_code
            #(#canister_declaration_children)*
        }
    }

    fn collect_children(&self) -> Vec<CanisterMethod> {
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
}
