use proc_macro2::TokenStream;

use crate::{
    generators::{candid_file_generation, random, vm_value_conversion},
    ToDeclarationTokenStream, ToTokenStream,
};
use node::{
    canister_method::{
        init_method, post_upgrade_method,
        {
            HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
            QueryMethod, UpdateMethod,
        },
    },
    data_type::{Array, Func, Primitive, Record, Tuple, TypeAlias, Variant},
    {data_type, external_canister, ActFunctionGuard, ExternalCanister},
};

use self::node::data_type::{deduplicate, new_deduplicate, type_alias};

pub mod actable;
pub mod node;

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub data_types: DataTypes,
    pub external_canisters: Vec<ExternalCanister>,
    pub function_guards: Vec<ActFunctionGuard>,
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
    pub arrays: Vec<Array>,
    pub funcs: Vec<Func>,
    pub options: Vec<data_type::Option>,
    pub primitives: Vec<Primitive>,
    pub records: Vec<Record>,
    pub tuples: Vec<Tuple>,
    pub type_aliases: Vec<TypeAlias>,
    pub variants: Vec<Variant>,
}

impl ToTokenStream<()> for AbstractCanisterTree {
    fn to_token_stream(&self, _: ()) -> TokenStream {
        let body = &self.body;
        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation(&self.cdk_name);

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.try_from_vm_value_impls;

        let func_arg_token = data_type::func::generate_func_arg_token();

        let cross_canister_functions =
            self.external_canisters
                .to_token_stream(external_canister::TokenStreamContext {
                    cdk_name: &self.cdk_name,
                    keyword_list: &self.keywords,
                });

        let heartbeat_method = self
            .canister_methods
            .heartbeat_method
            .to_token_stream(&self.cdk_name);
        let init_method =
            self.canister_methods
                .init_method
                .to_token_stream(init_method::TokenStreamContext {
                    cdk_name: &self.cdk_name,
                    keyword_list: &self.keywords,
                });
        let inspect_message_method = self
            .canister_methods
            .inspect_message_method
            .to_token_stream(&self.cdk_name);
        let post_upgrade_method = self.canister_methods.post_upgrade_method.to_token_stream(
            post_upgrade_method::TokenStreamContext {
                cdk_name: &self.cdk_name,
                keyword_list: &self.keywords,
            },
        );
        let pre_upgrade_method = self
            .canister_methods
            .pre_upgrade_method
            .to_token_stream(&self.cdk_name);

        let query_methods = self
            .canister_methods
            .query_methods
            .to_token_stream(&self.keywords);
        let update_methods = self
            .canister_methods
            .update_methods
            .to_token_stream(&self.keywords);
        let function_guards = self.function_guards.to_token_stream(&self.keywords);

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code(&self.cdk_name);

        let funcs = new_deduplicate(&self.data_types.funcs, &self.keywords);
        let records = new_deduplicate(&self.data_types.records, &self.keywords);
        let tuples = new_deduplicate(&self.data_types.tuples, &self.keywords);
        let type_aliases = new_deduplicate(&self.data_types.type_aliases, &self.keywords);
        let variants = new_deduplicate(&self.data_types.variants, &self.keywords);

        // let funcs: TokenStream = self.data_types.funcs.to_declaration(&self.keywords);
        let funcs: TokenStream = funcs.to_declaration(&self.keywords);
        let tuples: TokenStream = tuples.to_declaration(&self.keywords);
        let type_aliases: TokenStream = type_aliases.to_declaration(&self.keywords);
        let records: TokenStream = records.to_declaration(&self.keywords);
        let variants: TokenStream = variants.to_declaration(&self.keywords);
        // let records: TokenStream = self.data_types.records.to_declaration(&self.keywords);
        // let tuples: TokenStream = self.data_types.tuples.to_declaration(&self.keywords);
        // let type_aliases: TokenStream = self.data_types.type_aliases.to_declaration(&self.keywords);
        // let variants: TokenStream = self.data_types.variants.to_declaration(&self.keywords);

        quote::quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #heartbeat_method
            #init_method
            #inspect_message_method
            #post_upgrade_method
            #pre_upgrade_method

            #query_methods
            #update_methods
            #function_guards
            #func_arg_token

            #type_aliases
            #funcs
            #records
            #tuples
            #variants

            #cross_canister_functions

            #body

            #candid_file_generation_code
        }
    }
}
