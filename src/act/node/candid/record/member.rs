use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{candid::type_annotation::ToTypeAnnotation, CandidType},
    keyword,
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub candid_type: CandidType,
}

impl Member {
    pub fn to_token_stream(&self, keyword_list: &Vec<String>, prefix: String) -> TokenStream {
        let type_annotation = self.candid_type.to_type_annotation(keyword_list, prefix);
        let name = keyword::make_rust_safe(&self.name, keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&name, keyword_list);
        quote!(#rename_attr #name: Box<#type_annotation>)
    }
}
