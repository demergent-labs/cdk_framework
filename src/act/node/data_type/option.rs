use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::act::{declaration::ToDeclaration, node::traits::has_members::HasMembers};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug)]
pub struct Option {
    pub enclosed_type: Box<DataType>,
}

impl HasMembers for Option {
    fn get_members(&self) -> Vec<DataType> {
        vec![self.get_enclosed_type()]
    }

    fn create_member_prefix(&self, _: usize, _: String) -> String {
        format!("OptionEnclosedType")
    }
}

impl Option {
    pub fn get_enclosed_type(&self) -> DataType {
        *self.enclosed_type.clone()
    }
}

impl ToDeclaration<Vec<String>> for Option {
    fn create_code(&self, _: &Vec<String>, _: String) -> std::option::Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> std::option::Option<String> {
        None
    }

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}

impl ToTypeAnnotation<Vec<String>> for Option {
    fn to_type_annotation(&self, context: &Vec<String>, parental_prefix: String) -> TokenStream {
        let enclosed_type_annotation = self
            .enclosed_type
            .to_type_annotation(context, self.create_member_prefix(0, parental_prefix));
        quote!(Option<#enclosed_type_annotation>)
    }
}
