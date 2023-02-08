use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{FnParam, HasParams};
use crate::act::declaration::ToDeclaration;

#[derive(Clone)]
pub struct PostUpgradeMethod {
    pub params: Vec<FnParam>,
    pub body: TokenStream,
}

pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl HasParams for PostUpgradeMethod {
    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("PostUpgradeParamNum{}", param_index)
    }
}

impl ToDeclaration<TokenStreamContext<'_>> for PostUpgradeMethod {
    fn create_code(&self, context: &TokenStreamContext<'_>, _: String) -> Option<TokenStream> {
        let function_name = format_ident!("_{}_post_upgrade", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params = self.create_parameter_list_token_stream(context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::post_upgrade]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("PostUpgrade".to_string())
    }

    fn create_child_declarations(
        &self,
        context: &TokenStreamContext<'_>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        self.create_param_declarations(context.keyword_list)
    }
}
