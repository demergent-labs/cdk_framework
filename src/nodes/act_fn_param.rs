use proc_macro2::TokenStream;

use crate::{ActDataType, ToTokenStream};

use super::data_type_nodes::ToIdent;

// TODO Consider having access to both strings and idents as necessary

#[derive(Debug, Clone)]
pub struct ActFnParam {
    pub name: String,
    pub data_type: ActDataType,
}

impl ActFnParam {
    pub fn prefixed_name(&self) -> String {
        format!("_cdk_user_defined_{}", self.name)
    }
}

impl ToTokenStream<&Vec<String>> for ActFnParam {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let name = self.prefixed_name().to_identifier();
        let data_type = &self.data_type.to_token_stream(keyword_list);
        quote::quote! {
            #name: #data_type
        }
    }
}
