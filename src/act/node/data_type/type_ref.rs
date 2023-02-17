use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use super::traits::ToTypeAnnotation;
use crate::{act::proclamation::Proclaim, traits::ToIdent};

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTypeAnnotation<Vec<String>> for TypeRef {
    fn to_type_annotation(&self, _keyword_list: &Vec<String>, _: String) -> TokenStream {
        // TODO use the keyword list to make the identifier rust safe
        let ident = self.name.to_identifier().to_token_stream();
        quote!(#ident)
    }
}

impl Proclaim<Vec<String>> for TypeRef {
    fn create_declaration(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn collect_inline_declarations(
        &self,
        _: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
