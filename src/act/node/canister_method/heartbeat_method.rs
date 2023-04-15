use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::Context, Declaration, Declare},
    traits::WithUserDefinedPrefix,
};

#[derive(Clone)]
pub struct HeartbeatMethod {
    pub guard_function_name: Option<String>,
    pub body: TokenStream,
}

impl Declare<Context> for HeartbeatMethod {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        let body = &self.body;
        let macro_args = match &self.guard_function_name {
            Some(guard_function_name) => {
                let prefixed_guard_function_name = guard_function_name.with_user_defined_prefix();
                quote! {guard = #prefixed_guard_function_name}
            }
            None => quote!(),
        };

        Some(quote! {
            #[ic_cdk_macros::heartbeat(#macro_args)]
            fn heartbeat() {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![]
    }
}
