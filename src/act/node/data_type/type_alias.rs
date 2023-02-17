use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{node::traits::HasEnclosedType, proclamation::Proclaim},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<DataType>,
}

impl HasEnclosedType for TypeAlias {
    fn get_enclosed_type(&self) -> DataType {
        *self.aliased_type.clone()
    }
}

impl ToTypeAnnotation<Vec<String>> for TypeAlias {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl Proclaim<Vec<String>> for TypeAlias {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        let name = self.name.to_identifier();
        let alias = self.aliased_type.to_type_annotation(
            keyword_list,
            self.create_enclosed_type_prefix(parental_prefix, "TypeAlias".to_string()),
        );
        Some(quote!(type #name = #alias;))
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_enclosed_type_declaration(
            keyword_list,
            parental_prefix,
            "TypeAlias".to_string(),
        )
    }
}
