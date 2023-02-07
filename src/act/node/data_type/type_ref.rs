use std::collections::HashMap;

use crate::{act::node::declaration::ToDeclaration, traits::ToIdent};
use proc_macro2::TokenStream;
use quote::ToTokens;

use super::traits::ToTypeAnnotation;

// TODO what's more(see below) I think we don't even need it for the old version anymore
// TODO I think this is just temporary for that the old versions of kybra and azle will still compile

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTypeAnnotation<Vec<String>> for TypeRef {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToDeclaration<Vec<String>> for TypeRef {
    fn create_code(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
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
