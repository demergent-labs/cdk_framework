use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::CandidType, ToTypeAnnotation},
    traits::has_members::Member,
};

#[derive(Clone, Debug)]
pub struct Elem {
    pub index: usize,
    pub candid_type: CandidType,
}

impl Into<Member> for &Elem {
    fn into(self) -> Member {
        Member {
            name: self.index.to_string(),
            candid_type: self.candid_type.clone(),
        }
    }
}

impl Elem {
    pub fn to_tuple_elem_token_stream(
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
