use quote::{quote, ToTokens};

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{HasInlines, HasMembers, ToIdent},
    utils,
};

use super::Member;

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

impl ToTypeAnnotation<Context> for Record {
    fn to_type_annotation(&self, _: &Context, inline_name: String) -> TypeAnnotation {
        self.get_name(&inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Context> for Record {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        let record_ident = self.get_name(&inline_name).to_ident();
        let member_token_streams: Vec<_> = self
            .members
            .iter()
            .map(|member| {
                member.to_record_member_token_stream(context, self.get_name(&inline_name))
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
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(&inline_name), context)
    }
}

impl HasMembers for Record {
    fn get_members(&self) -> Vec<Member> {
        self.members.clone()
    }
}
