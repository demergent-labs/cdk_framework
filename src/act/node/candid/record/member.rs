use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::Context, ToTypeAnnotation},
    keyword,
    traits::ToIdent,
};

use super::Member;

impl Member {
    pub fn to_record_member_token_stream(
        &self,
        context: &Context,
        parent_name: String,
    ) -> TokenStream {
        let type_annotation = self.to_type_annotation(context, parent_name);
        let name = keyword::make_rust_safe(&self.name, &context.keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&name, &context.keyword_list);
        quote!(#rename_attr #name: Box<#type_annotation>)
    }
}
