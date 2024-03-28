use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::Context, Declaration, Declare};

#[derive(Clone)]
pub struct InspectMessageMethod {
    pub body: TokenStream,
}

impl Declare<Context> for InspectMessageMethod {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        let body = &self.body;

        Some(quote! {
            #[ic_cdk_macros::inspect_message]
            fn inspect_message() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![]
    }
}
