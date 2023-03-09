use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::{Declaration, Declare};

#[derive(Clone)]
pub struct InspectMessageMethod {
    pub body: TokenStream,
}

impl Declare<String> for InspectMessageMethod {
    fn to_declaration(&self, cdk_name: &String, _: String) -> Option<Declaration> {
        let function_name = format_ident!("_{}_inspect_message", cdk_name.to_lowercase(),);
        let body = &self.body;
        Some(quote! {
            #[ic_cdk_macros::inspect_message]
            fn #function_name() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &String, _: String) -> Vec<Declaration> {
        vec![]
    }
}
