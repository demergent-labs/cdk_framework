use proc_macro2::TokenStream;

use crate::{
    act::node::{data_type::traits::ToTypeAnnotation, DataType},
    traits::ToIdent,
};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_: DataType,
}

impl Param {
    pub fn prefixed_name(&self) -> String {
        format!("_cdk_user_defined_{}", self.name)
    }

    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        function_name: String,
    ) -> TokenStream {
        let name = self.prefixed_name().to_identifier();
        let type_annotation = &self.type_.to_type_annotation(
            keyword_list,
            format!("{}{}", function_name, self.prefixed_name()),
        );
        quote::quote! {
            #name: #type_annotation
        }
    }
}
