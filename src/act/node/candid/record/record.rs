use quote::{quote, ToTokens};

use crate::{
    act::{Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{has_members::Member, HasMembers, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Record {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

impl Record {
    fn get_name(&self, inline_name: &String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => utils::create_inline_name(inline_name),
        }
    }
}

impl ToTypeAnnotation<Vec<String>> for Record {
    fn to_type_annotation(&self, _: &Vec<String>, inline_name: String) -> TypeAnnotation {
        self.get_name(&inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for Record {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Option<Declaration> {
        let record_ident = self.get_name(&inline_name).to_ident();
        let member_token_streams: Vec<_> = self
            .members
            .iter()
            .map(|member| {
                member.to_record_member_token_stream(
                    keyword_list,
                    self.create_member_prefix(member, self.get_name(&inline_name)),
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

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.collect_member_inline_declarations(keyword_list, self.get_name(&inline_name))
    }
}

impl HasMembers for Record {
    fn get_members(&self) -> Vec<Member> {
        self.members.clone()
    }
}
