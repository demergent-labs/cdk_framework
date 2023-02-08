use std::collections::HashMap;

use crate::{act::declaration::ToDeclaration, traits::ToIdent};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, Clone)]
pub struct FunctionGuard {
    pub body: TokenStream,
    pub name: String,
}

impl ToDeclaration<Vec<String>> for FunctionGuard {
    fn create_code(&self, keyword_list: &Vec<String>, _: String) -> Option<TokenStream> {
        Some(self.generate_function(keyword_list))
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_child_declarations(
        &self,
        _: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        HashMap::new()
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
