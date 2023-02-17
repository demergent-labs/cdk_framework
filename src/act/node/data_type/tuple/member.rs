use proc_macro2::TokenStream;

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
        self.type_.to_type_annotation(keyword_list, member_prefix)
    }
}
