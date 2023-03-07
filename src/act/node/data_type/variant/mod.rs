use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{
    type_annotation::{ToTypeAnnotation, TypeAnnotation},
    DataType,
};
use crate::{
    act::node::{declaration::Declare, traits::HasMembers, Declaration},
    traits::ToIdent,
};

pub mod member;

pub use self::member::Member;

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

impl Variant {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Variant", parental_prefix),
        }
    }
}

impl<C> ToTypeAnnotation<C> for Variant {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TypeAnnotation {
        self.get_name(parental_prefix).to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for Variant {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<Declaration> {
        let variant_ident = self.get_name(parental_prefix.clone()).to_ident();
        let member_token_streams: Vec<TokenStream> = self
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
            enum #variant_ident {
                #(#member_token_streams),*
            }
        ))
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Vec<Declaration> {
        self.collect_member_inline_declarations(
            keyword_list,
            self.get_name(parental_prefix.clone()),
        )
    }
}

impl HasMembers for Variant {
    fn get_members(&self) -> Vec<DataType> {
        self.members
            .iter()
            .map(|member| member.type_.clone())
            .collect()
    }
}
