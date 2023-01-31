pub mod actable;
pub mod node;

use proc_macro2::TokenStream;

use node::canister_methods::{
    init_method, post_upgrade_method,
    {
        HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
        QueryMethod, UpdateMethod,
    },
};
use node::{data_types, external_canister, ActFunctionGuard, ExternalCanister};

use self::node::data_types::{
    ActArray, ActFunc, ActOption, ActPrimitive, ActRecord, ActTuple, ActTypeRef, ActVariant,
};

// TODO watch out for which is super and which is crate
use super::{
    generators::{candid_file_generation, random, vm_value_conversion},
    ToTokenStream, ToTokenStreams,
};

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub arrays: Vec<ActArray>,
    pub body: TokenStream,
    pub cdk_name: String,
    pub external_canisters: Vec<ExternalCanister>,
    pub funcs: Vec<ActFunc>,
    pub header: TokenStream,
    pub heartbeat_method: Option<HeartbeatMethod>,
    pub init_method: InitMethod,
    pub inspect_message_method: Option<InspectMessageMethod>,
    pub keywords: Vec<String>,
    pub options: Vec<ActOption>,
    pub post_upgrade_method: PostUpgradeMethod,
    pub pre_upgrade_method: PreUpgradeMethod,
    pub primitives: Vec<ActPrimitive>,
    pub query_methods: Vec<QueryMethod>,
    pub records: Vec<ActRecord>,
    pub try_from_vm_value_impls: TokenStream,
    pub try_into_vm_value_impls: TokenStream,
    pub tuples: Vec<ActTuple>,
    pub type_refs: Vec<ActTypeRef>,
    pub update_methods: Vec<UpdateMethod>,
    pub function_guards: Vec<ActFunctionGuard>,
    pub variants: Vec<ActVariant>,
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

        let func_arg_token = data_types::func::generate_func_arg_token();

        let cross_canister_functions =
            self.external_canisters
                .to_token_streams(external_canister::TokenStreamContext {
                    cdk_name: &self.cdk_name,
                    keyword_list: &self.keywords,
                });

        let heartbeat_method = self.heartbeat_method.to_token_stream(&self.cdk_name);
        let init_method = self
            .init_method
            .to_token_stream(init_method::TokenStreamContext {
                cdk_name: &self.cdk_name,
                keyword_list: &self.keywords,
            });
        let inspect_message_method = self.inspect_message_method.to_token_stream(&self.cdk_name);
        let post_upgrade_method =
            self.post_upgrade_method
                .to_token_stream(post_upgrade_method::TokenStreamContext {
                    cdk_name: &self.cdk_name,
                    keyword_list: &self.keywords,
                });
        let pre_upgrade_method = self.pre_upgrade_method.to_token_stream(&self.cdk_name);

        let query_methods = self.query_methods.to_token_streams(&self.keywords);
        let update_methods = self.update_methods.to_token_streams(&self.keywords);
        let function_guards = self.function_guards.to_token_streams(&self.keywords);

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code(&self.cdk_name);

        let arrays: Vec<TokenStream> = self.arrays.to_token_streams(&self.keywords);
        let funcs: Vec<TokenStream> = self
            .funcs
            .iter()
            .map(|act| act.to_token_stream(&self.keywords))
            .collect();
        let options: Vec<TokenStream> = self.options.to_token_streams(&self.keywords);
        let primitives: Vec<TokenStream> = self.primitives.to_token_streams(&self.keywords);
        let records: Vec<TokenStream> = self.records.to_token_streams(&self.keywords);
        let tuples: Vec<TokenStream> = self.tuples.to_token_streams(&self.keywords);
        let type_refs: Vec<TokenStream> = self.type_refs.to_token_streams(&self.keywords);
        let variants: Vec<TokenStream> = self.variants.to_token_streams(&self.keywords);

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

            #(#query_methods)*
            #(#update_methods)*
            #(#function_guards)*
            #func_arg_token

            #(#arrays)*
            #(#type_refs)*
            #(#funcs)*
            #(#options)*
            #(#primitives)*
            #(#records)*
            #(#tuples)*
            #(#variants)*

            #(#cross_canister_functions)*

            #body

            #candid_file_generation_code
        }
    }
}
