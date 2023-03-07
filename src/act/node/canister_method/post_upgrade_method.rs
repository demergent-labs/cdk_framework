use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    act::node::{declaration::Declare, param::Param, Context, Declaration},
    traits::HasParams,
};

#[derive(Clone)]
pub struct PostUpgradeMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl Declare<Context> for PostUpgradeMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_post_upgrade", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params = self.create_parameter_list_token_stream(&context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::post_upgrade]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.collect_param_inline_declarations(&context.keyword_list)
    }
}

impl HasParams for PostUpgradeMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_inline_prefix(&self) -> String {
        "PostUpgrade".to_string()
    }
}
