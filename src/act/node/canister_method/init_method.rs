use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::node::{
    declaration::Declare, param::Param, traits::HasParams, Context, Declaration,
};

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl Declare<Context> for InitMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_init", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params = self.create_parameter_list_token_stream(&context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.collect_param_inline_declarations(&context.keyword_list)
    }
}

impl HasParams for InitMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_inline_prefix(&self) -> String {
        "InitMethod".to_string()
    }
}
