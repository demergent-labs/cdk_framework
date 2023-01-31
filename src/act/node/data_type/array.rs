use super::{traits::HasMembers, DataType, LiteralOrTypeAlias};
use crate::{traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug)]
pub struct Array {
    pub act_type: LiteralOrTypeAlias<Literal, TypeAlias>,
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub enclosed_type: Box<DataType>,
}

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub enclosed_type: Box<DataType>,
}

impl HasMembers for Array {
    fn get_members(&self) -> Vec<DataType> {
        vec![self.get_enclosed_type()]
    }
}

impl Array {
    pub fn get_enclosed_type(&self) -> DataType {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => &*literal.enclosed_type,
            LiteralOrTypeAlias::TypeAlias(type_alias) => &*type_alias.enclosed_type,
        }
        .clone()
    }
}

impl ToTokenStream<&Vec<String>> for Literal {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(keyword_list);
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl ToTokenStream<&Vec<String>> for TypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let name = self.name.to_identifier();
        let enclosed_type = self.enclosed_type.to_token_stream(keyword_list);
        quote!(type #name = Vec<#enclosed_type>;)
    }
}

impl ToTokenStream<&Vec<String>> for Array {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.act_type.to_token_stream(context)
    }
}
