use proc_macro2::TokenStream;
use quote::quote;

use crate::{nodes::ActFnParam, ToTokenStream, ToTokenStreams};

#[derive(Clone)]
pub struct ActInitMethod {
    pub params: Vec<ActFnParam>,
    pub body: TokenStream,
}

impl ToTokenStream<&Vec<String>> for ActInitMethod {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        let body = &self.body;
        let params = &self.params.to_token_streams(context);
        quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn _azle_init(#(#params),*) {
                #body
            }
        }
    }
}
