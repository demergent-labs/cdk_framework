use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::Elem;
use crate::{
    act::{node::Member, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{HasDeclarableTypes, HasMembers, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub name: Option<String>,
    pub elems: Vec<Elem>,
}

impl Tuple {
    fn get_name(&self, inline_name: &String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => utils::create_inline_name(&inline_name),
        }
    }
}

impl<C> ToTypeAnnotation<C> for Tuple {
    fn to_type_annotation(&self, _: &C, inline_name: String) -> TypeAnnotation {
        self.get_name(&inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for Tuple {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Option<Declaration> {
        let tuple_ident = self.get_name(&inline_name).to_ident();
        let member_idents: Vec<TokenStream> = self
            .elems
            .iter()
            .enumerate()
            .map(|(index, elem)| {
                elem.to_tuple_elem_token_stream(index, &self.get_name(&inline_name), keyword_list)
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
            struct #tuple_ident (
                #member_idents
            );
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

impl HasMembers for Tuple {
    fn get_members(&self) -> Vec<Member> {
        self.elems
            .iter()
            .enumerate()
            .map(|(index, elem)| elem.to_member(index))
            .collect()
    }
}
