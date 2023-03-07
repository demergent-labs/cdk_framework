use quote::{quote, ToTokens};

use super::type_annotation::{ToTypeAnnotation, TypeAnnotation};
use crate::{
    act::node::{declaration::Declare, Declaration},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTypeAnnotation<Vec<String>> for TypeRef {
    fn to_type_annotation(&self, _keyword_list: &Vec<String>, _: String) -> TypeAnnotation {
        // TODO use the keyword list to make the identifier rust safe
        let ident = self.name.to_ident().to_token_stream();
        quote!(#ident)
    }
}

impl Declare<Vec<String>> for TypeRef {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<Declaration> {
        vec![]
    }
}
