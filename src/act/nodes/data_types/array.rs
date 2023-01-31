use super::{
    traits::{HasMembers, ToIdent},
    ActDataType, LiteralOrTypeAlias,
};
use crate::ToTokenStream;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug)]
pub struct ActArray {
    pub act_type: LiteralOrTypeAlias<ActArrayLiteral, ActArrayTypeAlias>,
}

#[derive(Clone, Debug)]
pub struct ActArrayLiteral {
    pub enclosed_type: Box<ActDataType>,
}

#[derive(Clone, Debug)]
pub struct ActArrayTypeAlias {
    pub name: String,
    pub enclosed_type: Box<ActDataType>,
}

impl HasMembers for ActArray {
    fn get_members(&self) -> Vec<ActDataType> {
        vec![self.get_enclosed_type()]
    }
}

impl ActArray {
    pub fn get_enclosed_type(&self) -> ActDataType {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => &*literal.enclosed_type,
            LiteralOrTypeAlias::TypeAlias(type_alias) => &*type_alias.enclosed_type,
        }
        .clone()
    }
}

impl ToTokenStream<&Vec<String>> for ActArrayLiteral {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(keyword_list);
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl ToTokenStream<&Vec<String>> for ActArrayTypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let name = self.name.to_identifier();
        let enclosed_type = self.enclosed_type.to_token_stream(keyword_list);
        quote!(type #name = Vec<#enclosed_type>;)
    }
}

impl ToTokenStream<&Vec<String>> for ActArray {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.act_type.to_token_stream(context)
    }
}
