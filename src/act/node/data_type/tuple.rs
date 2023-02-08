use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{declaration::ToDeclaration, node::traits::has_members::HasMembers},
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub name: Option<String>,
    pub elems: Vec<Elem>,
}

#[derive(Clone, Debug)]
pub struct Elem {
    pub elem_type: DataType,
}

impl Tuple {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Func", parental_prefix),
        }
    }
}

impl HasMembers for Tuple {
    fn get_members(&self) -> Vec<DataType> {
        self.elems
            .iter()
            .map(|elem| elem.elem_type.clone())
            .collect()
    }

    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String {
        format!("{}MemberNum{}", self.get_name(parental_prefix), index)
    }
}

impl<C> ToTypeAnnotation<C> for Tuple {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TokenStream {
        self.get_name(parental_prefix)
            .to_identifier()
            .to_token_stream()
    }
}

impl ToDeclaration<Vec<String>> for Tuple {
    fn create_code(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        let type_ident = self.get_name(parental_prefix.clone()).to_identifier();
        let elem_idents: Vec<TokenStream> = self
            .elems
            .iter()
            .enumerate()
            .map(|(index, elem)| {
                elem.to_token_stream(
                    keyword_list,
                    self.create_member_prefix(index, parental_prefix.clone()),
                )
            })
            .collect();

        let elem_idents = if elem_idents.len() == 1 {
            let elem_ident = &elem_idents[0];
            quote!((#elem_ident,))
        } else {
            quote!(#(#elem_idents),*)
        };

        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident (
                #elem_idents
            );
        ))
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        Some(self.get_name(parental_prefix))
    }

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}

impl Elem {
    fn to_token_stream(&self, keyword_list: &Vec<String>, tuple_name: String) -> TokenStream {
        if self.elem_type.needs_to_be_boxed() {
            let ident = self.elem_type.to_type_annotation(keyword_list, tuple_name);
            quote!(Box<#ident>)
        } else {
            quote!(self.elem_type.to_token_stream())
        }
    }
}
