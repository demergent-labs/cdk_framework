use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::generators::{candid_file_generation, random, vm_value_conversion};
use node::{
    canister_method::{
        init_method, post_upgrade_method,
        {
            HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
            QueryMethod, UpdateMethod,
        },
    },
    data_type::{new_deduplicate, Func, Record, Tuple, TypeAlias, Variant},
    full_declaration::{Declaration, ToDeclaration},
    {data_type, external_canister, ExternalCanister, FunctionGuard},
};

pub mod actable;
pub mod node;

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
    pub pre_upgrade_method: PreUpgradeMethod,
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

// impl ToDeclaration< {} // TOD probably to to declaration for this?
impl ToDeclaration<()> for AbstractCanisterTree {
    fn create_code(&self, _: &(), _: String) -> Option<TokenStream> {
        let body = &self.body;
        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation(&self.cdk_name);

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.try_from_vm_value_impls;

        let func_arg_token = data_type::func::generate_func_arg_token();

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code(&self.cdk_name);

        Some(quote::quote! {
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

    fn create_child_declarations(
        &self,
        context: &(),
        parental_prefix: String,
    ) -> HashMap<String, Declaration> {
        let result = HashMap::new();

        let init_method_declaration = self.canister_methods.init_method.create_declaration(
            &init_method::TokenStreamContext {
                keyword_list: &self.keywords,
                cdk_name: &self.cdk_name,
            },
            "InitMethod".to_string(),
        );
        let init_method = init_method_declaration.code;

        result
    }
}

impl AbstractCanisterTree {
    // TODO I want this thing to use the acts
    pub fn to_token_stream(&self, _: &()) -> TokenStream {
        // TODO all of these strings should actually be the AbstractCanisterTree's name, but also it shouldn't matter because none of these need the prefix
        // TODO is there a way to pass None when we don't use it? I don't think so because only the callee will know if it needs it or not
        let cross_canister_functions = self.external_canisters.create_declaration(
            &external_canister::TokenStreamContext {
                cdk_name: &self.cdk_name,
                keyword_list: &self.keywords,
            },
            "ExternalCanisters".to_string(),
        );
        let cross_canister_functions = cross_canister_functions.code;

        let heartbeat_method = self
            .canister_methods
            .heartbeat_method
            .create_declaration(&self.cdk_name, "HeartbeatMethod".to_string());
        let heartbeat_method = heartbeat_method.code;

        let inspect_message_method = self
            .canister_methods
            .inspect_message_method
            .create_declaration(&self.cdk_name, "InspectMessageMethod".to_string());
        let inspect_message_method = inspect_message_method.code;

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
        let post_upgrade_method = post_upgrade_method.code;

        let pre_upgrade_method = self
            .canister_methods
            .pre_upgrade_method
            .create_declaration(&self.cdk_name, "Canister".to_string());
        let pre_upgrade_method = pre_upgrade_method.code;

        let query_methods_full_declarations = self
            .canister_methods
            .query_methods
            .create_declaration(&self.keywords, "QueryMethod".to_string());
        let query_methods = if let Some(declaration) = query_methods_full_declarations.code {
            declaration
        } else {
            quote!()
        };
        let query_types = query_methods_full_declarations.children;
        let update_method_full_declarations = self
            .canister_methods
            .update_methods
            .create_declaration(&self.keywords, "UpdateMethod".to_string());
        let update_methods = update_method_full_declarations.code;

        let function_guards = self
            .function_guards
            .create_declaration(&self.keywords, "Canister".to_string());
        let function_guards = function_guards.code;

        let funcs = new_deduplicate(&self.data_types.funcs, "GlocalFunc".to_string())
            .create_code(&self.keywords, "GlobalFunc".to_string());
        let records = new_deduplicate(&self.data_types.records, "GlobalRecords".to_string())
            .create_code(&self.keywords, "GlobalRecord".to_string());
        let tuples = new_deduplicate(&self.data_types.tuples, "GlobalTuples".to_string())
            .create_code(&self.keywords, "GlobalTuples".to_string());
        let type_aliases =
            new_deduplicate(&self.data_types.type_aliases, "GlobalTypeAlias".to_string())
                .create_code(&self.keywords, "GlobalTypeAlias".to_string());
        let variants = new_deduplicate(&self.data_types.variants, "GlobalVariant".to_string())
            .create_code(&self.keywords, "GlobalVariant".to_string());

        quote::quote! {
            #heartbeat_method
            #inspect_message_method
            #post_upgrade_method
            #pre_upgrade_method

            #query_methods
            #update_methods
            #function_guards

            #type_aliases
            #funcs
            #records
            #tuples
            #variants

            #cross_canister_functions
        }
    }
}
