use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::{proclamation::Proclaim, Declaration};

#[derive(Clone)]
pub struct HeartbeatMethod {
    pub body: TokenStream,
}

impl Proclaim<String> for HeartbeatMethod {
    fn create_declaration(&self, cdk_name: &String, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_heartbeat", cdk_name.to_lowercase());
        let body = &self.body;
        Some(quote! {
            #[ic_cdk_macros::heartbeat]
            fn #function_name() {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some("HeartbeatMethod".to_string())
    }

    fn collect_inline_declarations(&self, _: &String, _: String) -> Vec<Declaration> {
        vec![]
    }
}
