use proc_macro2::TokenStream;
use quote::quote;

use super::Member;
use crate::{
    act::{
        node::{CandidType, Context},
        ToTypeAnnotation,
    },
    keyword,
    traits::ToIdent,
};

impl Member {
    pub fn to_variant_member_token_stream(
        &self,
        context: &Context,
        parent_name: String,
        module_name: &Option<String>,
    ) -> TokenStream {
        let member_type_token_stream = match self.candid_type.clone() {
            CandidType::Primitive(_) => {
                if self
                    .to_type_annotation(context, parent_name.clone(), module_name)
                    .to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream =
                        self.to_type_annotation(context, parent_name, module_name);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_annotation =
                    self.to_type_annotation(context, parent_name, module_name);
                quote!((Box<#member_type_annotation>))
            }
        };
        let member_name = keyword::make_rust_safe(&self.name, &context.keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&member_name, &context.keyword_list);
        quote! {pub #rename_attr #member_name #member_type_token_stream}
    }
}
