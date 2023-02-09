use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::act::{declaration::ToDeclaration, node::traits::HasMembers};

#[derive(Clone, Debug)]
pub struct Boxed {
    pub enclosed_type: Box<DataType>,
}

impl HasMembers for Boxed {
    fn get_members(&self) -> Vec<DataType> {
        vec![*self.enclosed_type.clone()]
    }

    fn create_member_prefix(&self, _: usize, parental_prefix: String) -> String {
        format!("{}Boxed", parental_prefix)
    }
}

impl ToTypeAnnotation<Vec<String>> for Boxed {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TokenStream {
        let enclosed_rust_ident = self
            .enclosed_type
            .to_type_annotation(keyword_list, self.create_member_prefix(0, parental_prefix));
        quote!(Box<#enclosed_rust_ident>)
    }
}

impl ToDeclaration<Vec<String>> for Boxed {
    fn create_code(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn create_child_declarations(
        &self,
        _: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        HashMap::new()
    }
}
