use proc_macro2::TokenStream;
use quote::quote;

use crate::{act::ToTypeAnnotation, keyword, traits::ToIdent};

use super::Member;

impl Member {
    pub fn to_record_member_token_stream(
        &self,
        keyword_list: &Vec<String>,
        parent_name: String,
    ) -> TokenStream {
        let type_annotation = self.to_type_annotation(keyword_list, parent_name);
        let name = keyword::make_rust_safe(&self.name, keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&name, keyword_list);
        quote!(#rename_attr #name: Box<#type_annotation>)
    }
}
