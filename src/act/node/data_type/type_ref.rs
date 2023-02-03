use std::collections::HashMap;

use crate::{act::node::full_declaration::ToDeclaration, traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::ToTokens;

use super::traits::ToTypeAnnotation;

// TODO what's more(see below) I think we don't even need it for the old version anymore
// TODO I think this is just temporary for that the old versions of kybra and azle will still compile

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTokenStream<Vec<String>> for TypeRef {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.to_type_annotation(context, "".to_string())
    }
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
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        HashMap::new()
    }
}
