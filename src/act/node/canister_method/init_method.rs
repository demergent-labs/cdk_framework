use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::{
    node::{param::Param, traits::HasParams, NodeContext},
    proclamation::Proclaim,
    Declaration,
};

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl InitMethod {
    fn get_name(&self) -> String {
        "InitMethod".to_string()
    }
}

impl Proclaim<NodeContext> for InitMethod {
    fn create_declaration(&self, context: &NodeContext, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_init", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params =
            self.create_parameter_list_token_stream(&context.keyword_list, &self.get_name());
        Some(quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.get_name())
    }

    fn collect_inline_declarations(&self, context: &NodeContext, _: String) -> Vec<Declaration> {
        self.collect_param_inline_declarations(&context.keyword_list, &self.get_name())
    }
}

impl HasParams for InitMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}
