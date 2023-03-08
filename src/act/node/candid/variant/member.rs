use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::CandidType, ToTypeAnnotation},
    keyword,
    traits::{has_members::Member, ToIdent},
};

impl Member {
    pub fn to_variant_member_token_stream(
        &self,
        keyword_list: &Vec<String>,
        member_prefix: String,
    ) -> TokenStream {
        let member_type_token_stream = match self.candid_type.clone() {
            CandidType::Primitive(_) => {
                if self
                    .candid_type
                    .to_type_annotation(keyword_list, member_prefix.clone())
                    .to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream = self
                        .candid_type
                        .to_type_annotation(keyword_list, member_prefix);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_annotation = self
                    .candid_type
                    .to_type_annotation(keyword_list, member_prefix);
                quote!((#member_type_annotation))
            }
        };
        let member_name = keyword::make_rust_safe(&self.name, keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote! {#rename_attr #member_name #member_type_token_stream}
    }
}
