use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::hash::Hash;

use self::{
    declaration::{Declaration, ToDeclaration},
    node::{
        canister_method::{
            init_method, post_upgrade_method,
            {
                HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod,
                PreUpgradeMethod, QueryMethod, UpdateMethod,
            },
        },
        data_type::{func, Func, Record, Tuple, TypeAlias, Variant},
        {external_canister, ExternalCanister, FunctionGuard},
    },
};
use crate::generators::{candid_file_generation, random, vm_value_conversion};

pub mod declaration;
pub mod node;
pub mod to_node;

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub data_types: DataTypes,
    pub external_canisters: Vec<ExternalCanister>,
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

pub struct DataTypes {
    pub funcs: Vec<Func>,
    pub records: Vec<Record>,
    pub tuples: Vec<Tuple>,
    pub type_aliases: Vec<TypeAlias>,
    pub variants: Vec<Variant>,
}

impl ToDeclaration<()> for AbstractCanisterTree {
    fn create_code(&self, _: &(), _: String) -> Option<TokenStream> {
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

    fn create_child_declarations(&self, _: &(), _: String) -> HashMap<String, TokenStream> {
        let result = HashMap::new();

        let init_method_declaration = self.canister_methods.init_method.create_declaration(
            &init_method::TokenStreamContext {
                keyword_list: &self.keywords,
                cdk_name: &self.cdk_name,
            },
            "InitMethod".to_string(),
        );
        let result = add_declaration_to_map(init_method_declaration, result);

        let cross_canister_functions = self.external_canisters.create_declaration(
            &external_canister::TokenStreamContext {
                cdk_name: &self.cdk_name,
                keyword_list: &self.keywords,
            },
            "ExternalCanisters".to_string(),
        );
        let result = add_declaration_to_map(cross_canister_functions, result);

        let heartbeat_method = self
            .canister_methods
            .heartbeat_method
            .create_declaration(&self.cdk_name, "HeartbeatMethod".to_string());
        let result = add_declaration_to_map(heartbeat_method, result);

        let inspect_message_method = self
            .canister_methods
            .inspect_message_method
            .create_declaration(&self.cdk_name, "InspectMessageMethod".to_string());
        let result = add_declaration_to_map(inspect_message_method, result);

        let post_upgrade_method = self
            .canister_methods
            .post_upgrade_method
            .create_declaration(
                &post_upgrade_method::TokenStreamContext {
                    cdk_name: &self.cdk_name,
                    keyword_list: &self.keywords,
                },
                "PostUpgradeMethod".to_string(),
            );
        let result = add_declaration_to_map(post_upgrade_method, result);

        let pre_upgrade_method_declaration = self
            .canister_methods
            .pre_upgrade_method
            .create_declaration(&self.cdk_name, "Canister".to_string());
        let result = add_declaration_to_map(pre_upgrade_method_declaration, result);

        let query_method_declarations = self
            .canister_methods
            .query_methods
            .create_declaration(&self.keywords, "QueryMethod".to_string());
        let result = add_declaration_to_map(query_method_declarations, result);

        let update_method_declarations = self
            .canister_methods
            .update_methods
            .create_declaration(&self.keywords, "UpdateMethod".to_string());
        let result = add_declaration_to_map(update_method_declarations, result);

        let function_guards = self
            .function_guards
            .create_declaration(&self.keywords, "Canister".to_string());
        let result = add_declaration_to_map(function_guards, result);

        result
    }
}

fn add_declaration_to_map(
    declaration: Declaration,
    map: HashMap<String, TokenStream>,
) -> HashMap<String, TokenStream> {
    let mut result = HashMap::new();
    result.extend(map);
    if let Some(identifier) = declaration.identifier {
        if let Some(code) = declaration.code {
            result.insert(identifier, code);
        }
    }
    result.extend(declaration.children);
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
        let canister_prefix = "Canister".to_string();

        let canister_declaration = self.create_declaration(&(), canister_prefix.clone());

        let canister_declaration_code = match canister_declaration.code {
            Some(code) => code,
            None => quote!(),
        };

        let function_declarations: Vec<_> = self
            .create_child_declarations(&(), canister_prefix.clone())
            .values()
            .cloned()
            .collect();

        quote! {
            #canister_declaration_code
            #(#function_declarations)*
        }
    }
}
