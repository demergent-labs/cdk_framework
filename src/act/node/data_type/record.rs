use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{node::traits::has_members::HasMembers, proclamation::Proclaim},
    keyword,
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Record {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub type_: DataType,
}

impl Member {
    fn to_token_stream(&self, keyword_list: &Vec<String>, prefix: String) -> TokenStream {
        let member_type_annotation = self.type_.to_type_annotation(keyword_list, prefix);
        let member_name = keyword::make_rust_safe(&self.name, keyword_list).to_identifier();
        let rename_attr = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote!(#rename_attr #member_name: #member_type_annotation)
    }
}

impl Record {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Record", parental_prefix),
        }
    }
}

impl HasMembers for Record {
    fn get_members(&self) -> Vec<DataType> {
        self.members
            .iter()
            .map(|member| member.type_.clone())
            .collect()
    }

    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String {
        format!("{}Member{}", self.get_name(parental_prefix), index)
    }
}

impl Proclaim<Vec<String>> for Record {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        let type_ident = self.get_name(parental_prefix.clone()).to_identifier();
        let member_token_streams: Vec<TokenStream> = self
            .members
            .iter()
            .enumerate()
            .map(|(index, member)| {
                member.to_token_stream(
                    keyword_list,
                    self.create_member_prefix(index, parental_prefix.clone()),
                )
            })
            .collect();
        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident {
                #(#member_token_streams),*
            }
        ))
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        Some(self.get_name(parental_prefix))
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, parental_prefix)
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
