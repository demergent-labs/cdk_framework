use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    act::{node::Member, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{HasInlineTypes, HasMembers, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

impl Variant {
    fn get_name(&self, inline_name: &String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => utils::create_inline_name(&inline_name),
        }
    }
}

impl<C> ToTypeAnnotation<C> for Variant {
    fn to_type_annotation(&self, _: &C, inline_name: String) -> TypeAnnotation {
        self.get_name(&inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for Variant {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Option<Declaration> {
        let variant_ident = self.get_name(&inline_name).to_ident();
        let member_token_streams: Vec<TokenStream> = self
            .members
            .iter()
            .map(|member| {
                member.to_variant_member_token_stream(keyword_list, self.get_name(&inline_name))
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
        inline_name: String,
    ) -> Vec<Declaration> {
        self.collect_inline_declarations_from(self.get_name(&inline_name), keyword_list)
    }
}

impl HasMembers for Variant {
    fn get_members(&self) -> Vec<Member> {
        self.members.clone()
    }
}
