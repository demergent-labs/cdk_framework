use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::FnParam;
use crate::act::{declaration::ToDeclaration, node::traits::has_params::HasParams};

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<FnParam>,
    pub body: TokenStream,
}

pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl HasParams for InitMethod {
    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("InitParamNum{}", param_index)
    }
}

impl ToDeclaration<TokenStreamContext<'_>> for InitMethod {
    fn create_code(&self, context: &TokenStreamContext<'_>, _: String) -> Option<TokenStream> {
        let function_name = format_ident!("_{}_init", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params = self.create_parameter_list_token_stream(context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("InitMethod".to_string())
    }

    fn create_child_declarations(
        &self,
        context: &TokenStreamContext<'_>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        self.create_param_declarations(context.keyword_list)
    }
}
