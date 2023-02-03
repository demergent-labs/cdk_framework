use std::collections::HashMap;

use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::{
    act::node::full_declaration::ToFullDeclaration, traits::ToIdent, ToDeclarationTokenStream,
    ToTokenStream,
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
}

impl ToDeclarationTokenStream<Vec<String>> for TypeAlias {
    fn to_declaration(&self, context: &Vec<String>, _: String) -> TokenStream {
        let name = self.name.to_identifier();
        let alias = self.aliased_type.to_token_stream(context);
        quote!(type #name = #alias;)
    }
}

impl ToTypeAnnotation<Vec<String>> for TypeAlias {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<Vec<String>> for TypeAlias {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.to_type_annotation(context, "".to_string())
    }
}

impl ToFullDeclaration<Vec<String>> for TypeAlias {
    fn create_declaration(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        Some(self.to_declaration(context, parental_prefix))
    }

    fn create_identifier(&self, parental_prefix: String) -> String {
        self.name.clone()
    }

    fn create_child_declarations(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        // TODO
        HashMap::new()
    }
}
