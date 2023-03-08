use quote::{quote, ToTokens};

use crate::{
    act::{node::CandidType, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::ToIdent,
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
        let alias = self
            .aliased_type
            .to_type_annotation(keyword_list, self.name.clone());
        Some(quote!(type #name = #alias;))
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> Vec<Declaration> {
        self.aliased_type.flatten(keyword_list, self.name.clone())
    }
}
