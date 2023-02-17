pub mod member;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{node::traits::HasMembers, proclamation::Proclaim, Declaration, TypeAnnotation},
    traits::ToIdent,
};

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
        self.get_name(parental_prefix)
            .to_identifier()
            .to_token_stream()
    }
}

impl Proclaim<Vec<String>> for Variant {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<Declaration> {
        let type_ident = self.get_name(parental_prefix.clone()).to_identifier();
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
            enum #type_ident {
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
        self.create_member_declarations(keyword_list, self.get_name(parental_prefix.clone()))
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
