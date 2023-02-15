use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use super::traits::ToTypeAnnotation;
use crate::{act::proclamation::Proclaim, traits::ToIdent};

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub enclosed_type: String,
}

impl ToTypeAnnotation<Vec<String>> for TypeRef {
    fn to_type_annotation(&self, keyword_list: &Vec<String>, _: String) -> TokenStream {
        let ident = self.enclosed_type.to_identifier().to_token_stream();
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
