use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::ToTokenStream;
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
}

impl Option {
    pub fn get_enclosed_type(&self) -> DataType {
        *self.enclosed_type.clone()
    }
}

impl ToTokenStream<Vec<String>> for Option {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.to_type_annotation(keyword_list, "".to_string())
    }
}

impl ToTypeAnnotation<Vec<String>> for Option {
    fn to_type_annotation(&self, context: &Vec<String>, _: String) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(context);
        quote!(Option<#enclosed_rust_ident>)
    }
}
