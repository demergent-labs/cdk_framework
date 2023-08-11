use std::usize;

use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{
    node::{CandidType, Context, Member},
    ToTypeAnnotation,
};

#[derive(Clone, Debug)]
pub struct Elem {
    pub candid_type: CandidType,
}

impl Elem {
    pub fn to_tuple_elem_token_stream(
        &self,
        index: usize,
        parent_name: &String,
        context: &Context,
        module_name: &Option<String>,
    ) -> TokenStream {
        let type_annotation =
            self.to_member(index)
                .to_type_annotation(context, parent_name.clone(), module_name);
        quote!(pub Box<#type_annotation>)
    }

    pub fn to_member(&self, index: usize) -> Member {
        Member {
            name: index.to_string(),
            candid_type: self.candid_type.clone(),
        }
    }
}
