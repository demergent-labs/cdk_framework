use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::act::{node::traits::HasMembers, proclamation::Proclaim};

#[derive(Clone, Debug)]
pub struct Opt {
    pub enclosed_type: Box<DataType>,
}

impl HasMembers for Opt {
    fn get_members(&self) -> Vec<DataType> {
        vec![self.get_enclosed_type()]
    }

    fn create_member_prefix(&self, _: usize, parental_prefix: String) -> String {
        format!("{}OptionEnclosedType", parental_prefix)
    }
}

impl Opt {
    pub fn get_enclosed_type(&self) -> DataType {
        *self.enclosed_type.clone()
    }
}

impl Proclaim<Vec<String>> for Opt {
    fn create_declaration(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}

impl ToTypeAnnotation<Vec<String>> for Opt {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TokenStream {
        let enclosed_type_annotation = self
            .enclosed_type
            .to_type_annotation(keyword_list, self.create_member_prefix(0, parental_prefix));
        quote!(Option<#enclosed_type_annotation>)
    }
}