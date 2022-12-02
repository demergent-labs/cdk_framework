use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ToTokenStream;

#[derive(Clone)]
pub struct ActInspectMessageMethod {
    pub body: TokenStream,
}

impl ToTokenStream<&String> for ActInspectMessageMethod {
    fn to_token_stream(&self, cdk_name: &String) -> TokenStream {
        let function_name = format_ident!("_{}_inspect_message", cdk_name.to_lowercase(),);
        let body = &self.body;
        quote! {
            #[ic_cdk_macros::inspect_message]
            fn #function_name() {
                #body
            }
        }
    }
}
