use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use super::{CanisterMethodContext, FnParam};
use crate::act::{node::traits::HasParams, proclamation::Proclaim};

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<FnParam>,
    pub body: TokenStream,
}

impl HasParams for InitMethod {
    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("InitParamNum{}", param_index)
    }
}

impl Proclaim<CanisterMethodContext> for InitMethod {
    fn create_declaration(
        &self,
        context: &CanisterMethodContext,
        _: String,
    ) -> Option<TokenStream> {
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

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("InitMethod".to_string())
    }

    fn create_inline_declarations(
        &self,
        context: &CanisterMethodContext,
        _: String,
    ) -> HashMap<String, TokenStream> {
        self.create_param_declarations(&context.keyword_list)
    }
}
