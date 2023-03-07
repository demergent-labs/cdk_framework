use quote::{quote, ToTokens};

use super::{
    type_annotation::{ToTypeAnnotation, TypeAnnotation},
    DataType,
};
use crate::{
    act::node::{proclamation::Proclaim, traits::has_members::HasMembers, Declaration},
    traits::ToIdent,
};

pub mod member;

pub use self::member::Member;

#[derive(Clone, Debug)]
pub struct Record {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

impl Record {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Record", parental_prefix),
        }
    }
}

impl ToTypeAnnotation<Vec<String>> for Record {
    fn to_type_annotation(&self, _: &Vec<String>, parental_prefix: String) -> TypeAnnotation {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Record", parental_prefix),
        }
        .to_ident()
        .to_token_stream()
    }
}

impl Proclaim<Vec<String>> for Record {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<Declaration> {
        let record_ident = self.get_name(parental_prefix.clone()).to_ident();
        let member_token_streams: Vec<_> = self
            .members
            .iter()
            .enumerate()
            .map(|(index, member)| {
                member.to_token_stream(
                    keyword_list,
                    self.create_member_prefix(index, self.get_name(parental_prefix.clone())),
                )
            })
            .collect();
        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #record_ident {
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
    ) -> Vec<Declaration> {
        self.collect_member_inline_declarations(keyword_list, self.get_name(parental_prefix))
    }
}

impl HasMembers for Record {
    fn get_members(&self) -> Vec<DataType> {
        self.members
            .iter()
            .map(|member| member.type_.clone())
            .collect()
    }
}
