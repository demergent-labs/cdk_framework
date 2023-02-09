use proc_macro2::TokenStream;

use crate::{
    act::node::{data_type::traits::ToTypeAnnotation, DataType},
    traits::ToIdent,
};

// TODO Consider having access to both strings and idents as necessary

#[derive(Debug, Clone)]
pub struct FnParam {
    pub name: String,
    pub data_type: DataType,
}

impl FnParam {
    pub fn prefixed_name(&self) -> String {
        format!("_cdk_user_defined_{}", self.name)
    }

    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        function_name: String,
    ) -> TokenStream {
        let name = self.prefixed_name().to_identifier();
        let type_annotation = &self.data_type.to_type_annotation(
            keyword_list,
            format!("{}{}", function_name, self.prefixed_name()),
        );
        quote::quote! {
            #name: #type_annotation
        }
    }
}
