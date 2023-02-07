use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::node::declaration::ToDeclaration;

#[derive(Clone)]
pub struct PreUpgradeMethod {
    pub body: TokenStream,
}

impl ToDeclaration<String> for PreUpgradeMethod {
    fn create_code(&self, cdk_name: &String, _: String) -> Option<TokenStream> {
        let function_name = format_ident!("_{}_pre_upgrade", cdk_name.to_lowercase());
        let body = &self.body;
        Some(quote! {
            #[ic_cdk_macros::pre_upgrade]
            fn #function_name() {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("PreUpgradeMethod".to_string())
    }

    fn create_child_declarations(&self, _: &String, _: String) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
