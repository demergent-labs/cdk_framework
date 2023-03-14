use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::{Declaration, Declare};

#[derive(Clone)]
pub struct HeartbeatMethod {
    pub guard_function_name: Option<String>,
    pub body: TokenStream,
}

impl Declare<String> for HeartbeatMethod {
    fn to_declaration(&self, cdk_name: &String, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_heartbeat", cdk_name.to_lowercase());
        let body = &self.body;
        let macro_args = match &self.guard_function_name {
            Some(guard_function_name) => quote! {guard = #guard_function_name},
            None => quote!(),
        };

        Some(quote! {
            #[ic_cdk_macros::heartbeat(#macro_args)]
            fn #function_name() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &String, _: String) -> Vec<Declaration> {
        vec![]
    }
}
