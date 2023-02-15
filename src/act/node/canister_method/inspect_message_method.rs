use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::act::proclamation::Proclaim;

#[derive(Clone)]
pub struct InspectMessageMethod {
    pub body: TokenStream,
}

impl Proclaim<String> for InspectMessageMethod {
    fn create_declaration(&self, cdk_name: &String, _: String) -> Option<TokenStream> {
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

    fn collect_inline_declarations(&self, _: &String, _: String) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
