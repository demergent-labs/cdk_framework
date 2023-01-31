use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ToTokenStream;

#[derive(Clone)]
pub struct HeartbeatMethod {
    pub body: TokenStream,
}

impl ToTokenStream<&String> for HeartbeatMethod {
    fn to_token_stream(&self, cdk_name: &String) -> TokenStream {
        let function_name = format_ident!("_{}_heartbeat", cdk_name.to_lowercase());
        let body = &self.body;
        quote! {
            #[ic_cdk_macros::heartbeat]
            fn #function_name() {
                #body
            }
        }
    }
}
