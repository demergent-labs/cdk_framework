use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::{act::node::full_declaration::ToDeclaration, ToTokenStream};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug)]
pub struct Array {
    pub enclosed_type: Box<DataType>,
}

impl HasMembers for Array {
    fn get_members(&self) -> Vec<DataType> {
        vec![*self.enclosed_type.clone()]
    }

    fn create_member_prefix(&self, _: usize, parental_prefix: String) -> String {
        format!("{}ArrayOf", parental_prefix)
    }
}

impl ToTokenStream<Vec<String>> for Array {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.to_type_annotation(keyword_list, "ArrayOf".to_string())
    }
}

impl ToTypeAnnotation<Vec<String>> for Array {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TokenStream {
        let enclosed_rust_ident = self
            .enclosed_type
            .to_type_annotation(keyword_list, self.create_member_prefix(0, parental_prefix));
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl ToDeclaration<Vec<String>> for Array {
    fn create_code(&self, _: &Vec<String>, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}
