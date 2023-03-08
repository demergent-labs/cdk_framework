use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::CandidType, ToTypeAnnotation};

#[derive(Clone, Debug)]
pub struct Member {
    pub candid_type: CandidType,
}

impl Member {
    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        member_prefix: String,
    ) -> TokenStream {
        let type_annotation = self
            .candid_type
            .to_type_annotation(keyword_list, member_prefix);
        quote!(Box<#type_annotation>)
    }
}
