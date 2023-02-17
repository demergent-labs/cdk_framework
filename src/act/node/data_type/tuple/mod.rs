pub mod member;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{
        node::traits::has_members::HasMembers, proclamation::Proclaim, Declaration, TypeAnnotation,
    },
    traits::ToIdent,
};

pub use self::member::Member;

#[derive(Clone, Debug)]
pub struct Tuple {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

impl Tuple {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Tuple", parental_prefix),
        }
    }
}

impl HasMembers for Tuple {
    fn get_members(&self) -> Vec<DataType> {
        self.members.iter().map(|elem| elem.type_.clone()).collect()
    }
}

impl<C> ToTypeAnnotation<C> for Tuple {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TypeAnnotation {
        self.get_name(parental_prefix)
            .to_identifier()
            .to_token_stream()
    }
}

impl Proclaim<Vec<String>> for Tuple {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<Declaration> {
        let type_ident = self.get_name(parental_prefix.clone()).to_identifier();
        let member_idents: Vec<TokenStream> = self
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

        let member_idents = if member_idents.len() == 1 {
            let member_ident = &member_idents[0];
            quote!((#member_ident,))
        } else {
            quote!(#(#member_idents),*)
        };

        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident (
                #member_idents
            );
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
        self.create_member_declarations(keyword_list, self.get_name(parental_prefix))
    }
}
