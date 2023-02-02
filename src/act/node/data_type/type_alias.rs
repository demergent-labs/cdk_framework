use super::{traits::HasMembers, DataType};
use crate::{traits::ToIdent, ToDeclarationTokenStream, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<DataType>,
}

impl HasMembers for TypeAlias {
    fn get_members(&self) -> Vec<DataType> {
        vec![*self.aliased_type.clone()]
    }
}

impl ToDeclarationTokenStream<&Vec<String>> for TypeAlias {
    fn to_declaration(&self, _: &Vec<String>) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<&Vec<String>> for TypeAlias {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        let name = self.name.to_identifier();
        let alias = self.aliased_type.to_token_stream(context);
        quote!(type #name = #alias;)
    }
}
