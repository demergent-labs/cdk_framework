use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{Context, Param},
        Declaration, Declare,
    },
    traits::{HasDeclarableTypes, HasParams, ToIdent},
};

#[derive(Clone)]
pub struct PostUpgradeMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl PostUpgradeMethod {
    fn get_name(&self, context: &Context) -> String {
        format!("_{}_post_upgrade", context.cdk_name.to_lowercase())
    }
}

impl Declare<Context> for PostUpgradeMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = self.get_name(context).to_ident();
        let body = &self.body;
        let params =
            self.create_parameter_list_token_stream(&self.get_name(context), &context.keyword_list);
        Some(quote! {
            #[ic_cdk_macros::post_upgrade]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.collect_inline_declarations_from(self.get_name(context), &context.keyword_list)
    }
}

impl HasParams for PostUpgradeMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}
