use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::Context, Declaration, Declare};

#[derive(Clone)]
pub struct HeartbeatMethod {
    pub body: TokenStream,
}

impl Declare<Context> for HeartbeatMethod {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        let body = &self.body;

        Some(quote! {
            #[ic_cdk_macros::heartbeat]
            fn heartbeat() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![]
    }
}
