use proc_macro2::TokenStream;

use crate::{ActDataType, ToTokenStream};

use super::data_type_nodes::ToIdent;

// TODO Consider having access to both strings and idents as necessary

#[derive(Debug, Clone)]
pub struct ActFnParam {
    pub name: String,
    pub data_type: ActDataType,
}

impl ToTokenStream<&Vec<String>> for ActFnParam {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        let name = self.name.to_identifier();
        let data_type = &self.data_type.to_token_stream(context);
        quote::quote! {
            #name: #data_type
        }
    }
}
