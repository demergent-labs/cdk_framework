use quote::quote;

use crate::act::{node::CandidType, Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

#[derive(Clone, Debug)]
pub struct Array {
    pub enclosed_type: Box<CandidType>,
}

impl ToTypeAnnotation<Vec<String>> for Array {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TypeAnnotation {
        let enclosed_rust_ident = self
            .enclosed_type
            .to_type_annotation(keyword_list, parental_prefix);
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl Declare<Vec<String>> for Array {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Vec<Declaration> {
        self.enclosed_type.flatten(keyword_list, parental_prefix)
    }
}
