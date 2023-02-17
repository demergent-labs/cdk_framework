use proc_macro2::TokenStream;
use quote::quote;

use crate::{act::proclamation::Proclaim, traits::ToIdent};

#[derive(Debug, Clone)]
pub struct GuardFunction {
    pub body: TokenStream,
    pub name: String,
}

impl Proclaim<Vec<String>> for GuardFunction {
    fn create_declaration(&self, _keyword_list: &Vec<String>, _: String) -> Option<TokenStream> {
        // TODO we will eventually need that _keyword list for when we analyze function names for keywords
        let name = self.name.to_identifier();
        let body = &self.body;

        Some(quote! {
            fn #name() -> Result<(), String> {
                #body
            }
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<TokenStream> {
        vec![]
    }
}
