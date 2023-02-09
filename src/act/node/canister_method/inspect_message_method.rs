use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::act::declaration::ToDeclaration;

#[derive(Clone)]
pub struct InspectMessageMethod {
    pub body: TokenStream,
}

impl ToDeclaration<String> for InspectMessageMethod {
    fn create_code(&self, cdk_name: &String, _: String) -> Option<TokenStream> {
        let function_name = format_ident!("_{}_inspect_message", cdk_name.to_lowercase(),);
        let body = &self.body;
        Some(quote! {
            #[ic_cdk_macros::inspect_message]
            fn #function_name() {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("InspectMessageMethod".to_string())
    }

    fn create_child_declarations(&self, _: &String, _: String) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
