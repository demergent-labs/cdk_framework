use super::{ActDataType, HasMembers, LiteralOrTypeAlias, ToIdent};
use crate::ToTokenStream;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Clone, Debug)]
pub struct ActOption {
    pub act_type: LiteralOrTypeAlias<ActOptionLiteral, ActOptionTypeAlias>,
}

#[derive(Clone, Debug)]
pub struct ActOptionLiteral {
    pub enclosed_type: Box<ActDataType>,
}

#[derive(Clone, Debug)]
pub struct ActOptionTypeAlias {
    pub name: String,
    pub enclosed_type: Box<ActDataType>,
}

impl HasMembers for ActOption {
    fn get_members(&self) -> Vec<ActDataType> {
        vec![self.get_enclosed_type()]
    }
}

impl ActOption {
    pub fn get_enclosed_type(&self) -> ActDataType {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => *literal.enclosed_type.clone(),
            LiteralOrTypeAlias::TypeAlias(type_alias) => *type_alias.enclosed_type.clone(),
        }
    }
}

impl ToTokenStream<&Vec<String>> for ActOption {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => literal.to_token_stream(keyword_list),
            LiteralOrTypeAlias::TypeAlias(type_alias) => type_alias.to_token_stream(keyword_list),
        }
    }
}

impl ToTokenStream<&Vec<String>> for ActOptionLiteral {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let enclosed_rust_ident = self.enclosed_type.to_token_stream(keyword_list);
        quote!(Option<#enclosed_rust_ident>)
    }
}

impl ToTokenStream<&Vec<String>> for ActOptionTypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let name = self.name.to_identifier();
        let enclosed_type = self.enclosed_type.to_token_stream(keyword_list);
        quote!(type #name = Option<#enclosed_type>;)
    }
}
