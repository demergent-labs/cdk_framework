use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{data_type::traits::ToTypeAnnotation, DataType},
    keyword,
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub type_: DataType,
}

impl Member {
    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        member_prefix: String,
    ) -> TokenStream {
        let member_type_token_stream = match self.type_.clone() {
            DataType::Primitive(_) => {
                if self
                    .type_
                    .to_type_annotation(keyword_list, member_prefix.clone())
                    .to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream =
                        self.type_.to_type_annotation(keyword_list, member_prefix);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_annotation =
                    self.type_.to_type_annotation(keyword_list, member_prefix);
                quote!((#member_type_annotation))
            }
        };
        let member_name = keyword::make_rust_safe(&self.name, keyword_list).to_identifier();
        let rename_attr = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote! {#rename_attr #member_name #member_type_token_stream}
    }
}
