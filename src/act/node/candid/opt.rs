use quote::quote;

use super::{
    type_annotation::{ToTypeAnnotation, TypeAnnotation},
    CandidType,
};
use crate::act::node::{declaration::Declare, traits::HasEnclosedType, Declaration};

#[derive(Clone, Debug)]
pub struct Opt {
    pub enclosed_type: Box<CandidType>,
}

impl ToTypeAnnotation<Vec<String>> for Opt {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TypeAnnotation {
        let enclosed_type_annotation = self.enclosed_type.to_type_annotation(
            keyword_list,
            self.create_enclosed_type_prefix(parental_prefix, "Opt".to_string()),
        );
        quote!(Option<#enclosed_type_annotation>)
    }
}

impl Declare<Vec<String>> for Opt {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Vec<Declaration> {
        self.collect_enclosed_type_inline_declaration(
            keyword_list,
            parental_prefix,
            "Opt".to_string(),
        )
    }
}

impl HasEnclosedType for Opt {
    fn get_enclosed_type(&self) -> CandidType {
        *self.enclosed_type.clone()
    }
}
