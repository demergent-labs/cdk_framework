use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::{act::proclamation::Proclaim, traits::ToIdent};

#[derive(Debug, Clone)]
pub struct FunctionGuard {
    pub body: TokenStream,
    pub name: String,
}

impl Proclaim<Vec<String>> for FunctionGuard {
    fn create_declaration(&self, _keyword_list: &Vec<String>, _: String) -> Option<TokenStream> {
        // TODO we will eventually need that _keyword list for when we analyze function names for keywords
        let function_name = self.name.to_identifier();
        let function_body = &self.body;

        Some(quote! {
            fn #function_name() -> Result<(), String> {
                #function_body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_inline_declarations(
        &self,
        _: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
