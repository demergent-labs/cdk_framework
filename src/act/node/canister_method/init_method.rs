use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{param::Param, Context},
        Declaration, Declare,
    },
    traits::{HasParams, ToIdent},
};

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl InitMethod {
    fn get_name(&self, context: &Context) -> String {
        format!("_{}_init", context.cdk_name.to_lowercase())
    }
}

impl Declare<Context> for InitMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = self.get_name(context).to_ident();
        let body = &self.body;
        let params = self
            .create_parameter_list_token_stream(&self.get_name(&context), &context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.collect_param_inline_declarations(&self.get_name(&context), &context.keyword_list)
    }
}

impl HasParams for InitMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}
