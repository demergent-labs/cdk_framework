use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::ToTokenStream;
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
}

impl ToTokenStream<Vec<String>> for Array {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.to_type_annotation(keyword_list, "".to_string())
    }
}

impl ToTypeAnnotation<Vec<String>> for Array {
    fn to_type_annotation(&self, context: &Vec<String>, _: String) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(context);
        quote!(Vec<#enclosed_rust_ident>)
    }
}
