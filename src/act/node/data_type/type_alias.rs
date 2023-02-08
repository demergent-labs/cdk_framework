use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{declaration::ToDeclaration, node::traits::has_members::HasMembers},
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<DataType>,
}

impl HasMembers for TypeAlias {
    fn get_members(&self) -> Vec<DataType> {
        vec![*self.aliased_type.clone()]
    }

    fn create_member_prefix(&self, _: usize, _: String) -> String {
        format!("{}AliasedType", self.name)
    }
}

impl ToTypeAnnotation<Vec<String>> for TypeAlias {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToDeclaration<Vec<String>> for TypeAlias {
    fn create_code(&self, context: &Vec<String>, parental_prefix: String) -> Option<TokenStream> {
        let name = self.name.to_identifier();
        let alias = self
            .aliased_type
            .to_type_annotation(context, parental_prefix);
        Some(quote!(type #name = #alias;))
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}
