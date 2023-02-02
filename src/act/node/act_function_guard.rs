use crate::{traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
pub struct FunctionGuard {
    pub body: TokenStream,
    pub name: String,
}

impl ToTokenStream<&Vec<String>> for FunctionGuard {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.generate_function(keyword_list)
    }
}

impl FunctionGuard {
    pub fn generate_function(&self, _keyword_list: &Vec<String>) -> TokenStream {
        // TODO we will eventually need that _keyword list for when we analyze function names for keywords
        let function_name = self.name.to_identifier();
        let function_body = &self.body;

        quote! {
            fn #function_name() -> Result<(), String> {
                #function_body
            }
        }
    }
}
