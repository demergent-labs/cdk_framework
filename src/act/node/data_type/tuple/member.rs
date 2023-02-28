use proc_macro2::TokenStream;
use quote::quote;

use crate::act::node::{data_type::type_annotation::ToTypeAnnotation, DataType};

#[derive(Clone, Debug)]
pub struct Member {
    pub type_: DataType,
}

impl Member {
    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        member_prefix: String,
    ) -> TokenStream {
        let type_annotation = self.type_.to_type_annotation(keyword_list, member_prefix);
        quote!(Box<#type_annotation>)
    }
}
