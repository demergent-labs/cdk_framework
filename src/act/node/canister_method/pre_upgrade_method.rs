use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::node::{declaration::Declare, Declaration};

#[derive(Clone)]
pub struct PreUpgradeMethod {
    pub body: TokenStream,
}

impl Declare<String> for PreUpgradeMethod {
    fn to_declaration(&self, cdk_name: &String, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_pre_upgrade", cdk_name.to_lowercase());
        let body = &self.body;
        Some(quote! {
            #[ic_cdk_macros::pre_upgrade]
            fn #function_name() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &String, _: String) -> Vec<Declaration> {
        vec![]
    }
}
