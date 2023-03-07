use quote::{quote, ToTokens};

use super::{
    type_annotation::{ToTypeAnnotation, TypeAnnotation},
    CandidType,
};
use crate::{
    act::node::{declaration::Declare, Declaration},
    traits::{HasEnclosedType, ToIdent},
};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<CandidType>,
}

impl ToTypeAnnotation<Vec<String>> for TypeAlias {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TypeAnnotation {
        self.name.to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for TypeAlias {
    fn to_declaration(&self, keyword_list: &Vec<String>, _: String) -> Option<Declaration> {
        let name = self.name.to_ident();
        let alias = self.aliased_type.to_type_annotation(
            keyword_list,
            self.create_enclosed_type_prefix(self.name.clone(), "TypeAlias".to_string()),
        );
        Some(quote!(type #name = #alias;))
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> Vec<Declaration> {
        self.collect_enclosed_type_inline_declaration(
            keyword_list,
            self.name.clone(),
            "TypeAlias".to_string(),
        )
    }
}

impl HasEnclosedType for TypeAlias {
    fn get_enclosed_type(&self) -> CandidType {
        *self.aliased_type.clone()
    }
}
