use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    act::{
        node::{candid::type_param::TypeParams, Context, Member},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasInlines, HasMembers, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: Option<String>,
    pub members: Vec<Member>,
    pub type_params: TypeParams,
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

impl Declare<Context> for Variant {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        let variant_ident = self.get_name(&inline_name).to_ident();
        let member_token_streams: Vec<TokenStream> = self
            .members
            .iter()
            .map(|member| {
                member.to_variant_member_token_stream(context, self.get_name(&inline_name))
            })
            .collect();
        let type_params_token_stream = self.type_params.get_type_params_token_stream();
        let where_clause_token_stream = self.type_params.get_where_clause_token_stream();

        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            enum #variant_ident #type_params_token_stream #where_clause_token_stream
            {
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

impl HasMembers for Variant {
    fn get_members(&self) -> Vec<Member> {
        self.members.clone()
    }
}
