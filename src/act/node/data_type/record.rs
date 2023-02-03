use std::{collections::HashMap, task::Context};

use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::{
    act::node::full_declaration::ToFullDeclaration, keyword, traits::ToIdent,
    ToDeclarationTokenStream, ToTokenStream,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Record {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub member_name: String,
    pub member_type: DataType,
}

impl HasMembers for Record {
    fn get_members(&self) -> Vec<DataType> {
        self.members
            .iter()
            .map(|member| member.member_type.clone())
            .collect()
    }
}

impl ToFullDeclaration<Vec<String>> for Record {
    fn create_declaration(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        let type_ident = self.create_identifier(parental_prefix).to_identifier();
        let member_token_streams: Vec<TokenStream> = self
            .members
            .iter()
            .map(|member| member.to_token_stream(context))
            .collect();
        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident {
                #(#member_token_streams),*
            }
        ))
    }

    fn create_identifier(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Record", parental_prefix),
        }
    }

    fn create_child_declarations(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, crate::act::node::full_declaration::Declaration> {
        todo!()
    }
}

impl ToDeclarationTokenStream<Vec<String>> for Record {
    fn to_declaration(&self, context: &Vec<String>, parental_prefix: String) -> TokenStream {
        match self.create_declaration(context, parental_prefix) {
            Some(declaration) => declaration,
            None => quote!(),
        }
    }
}

impl ToTokenStream<Vec<String>> for Record {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.to_type_annotation(context, "".to_string())
    }
}

impl ToTypeAnnotation<Vec<String>> for Record {
    fn to_type_annotation(&self, _: &Vec<String>, parental_prefix: String) -> TokenStream {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Record", parental_prefix),
        }
        .to_identifier()
        .to_token_stream()
    }
}

impl ToTokenStream<Vec<String>> for Member {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let member_type_token_stream = if self.member_type.needs_to_be_boxed() {
            let ident = self.member_type.to_token_stream(keyword_list);
            quote!(Box<#ident>)
        } else {
            quote!(self.member_type.to_token_stream())
        };
        let member_name = keyword::make_rust_safe(&self.member_name, keyword_list).to_identifier();
        let rename = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote!(#rename#member_name: #member_type_token_stream)
    }
}
