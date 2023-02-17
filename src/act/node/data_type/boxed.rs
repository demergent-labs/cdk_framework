use quote::{quote, ToTokens};

use super::type_annotation::{ToTypeAnnotation, TypeAnnotation};
use crate::{
    act::{proclamation::Proclaim, Declaration},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Boxed {
    pub enclosed_type: String,
}

impl ToTypeAnnotation<Vec<String>> for Boxed {
    fn to_type_annotation(&self, _keyword_list: &Vec<String>, _: String) -> TypeAnnotation {
        // TODO use the keyword list
        let ident = self.enclosed_type.to_identifier().to_token_stream();
        quote!(Box<#ident>)
    }
}

impl Proclaim<Vec<String>> for Boxed {
    fn create_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<Declaration> {
        vec![]
    }
}
