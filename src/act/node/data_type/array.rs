use super::{traits::HasMembers, DataType};
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

impl ToTokenStream<&Vec<String>> for Array {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(keyword_list);
        quote!(Vec<#enclosed_rust_ident>)
    }
}
