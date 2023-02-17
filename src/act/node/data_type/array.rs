use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::act::{node::traits::HasEnclosedType, proclamation::Proclaim};

#[derive(Clone, Debug)]
pub struct Array {
    pub enclosed_type: Box<DataType>,
}

impl HasEnclosedType for Array {
    fn get_enclosed_type(&self) -> DataType {
        *self.enclosed_type.clone()
    }
}

impl ToTypeAnnotation<Vec<String>> for Array {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_type_annotation(
            keyword_list,
            self.create_enclosed_type_prefix(parental_prefix, "Array".to_string()),
        );
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl Proclaim<Vec<String>> for Array {
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
        self.create_enclosed_type_declaration(keyword_list, parental_prefix, "Array".to_string())
    }
}
