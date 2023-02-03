use std::collections::HashMap;

use crate::{act::node::full_declaration::ToFullDeclaration, traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::ToTokens;

// TODO what's more(see below) I think we don't even need it for the old version anymore
// TODO I think this is just temporary for that the old versions of kybra and azle will still compile

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTokenStream<&Vec<String>> for TypeRef {
    fn to_token_stream(&self, _: &Vec<String>) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToFullDeclaration<Vec<String>> for TypeRef {
    fn create_declaration(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> String {
        self.name.clone()
    }

    fn create_child_declarations(
        &self,
        _: &Vec<String>,
        _: String,
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        HashMap::new()
    }
}
