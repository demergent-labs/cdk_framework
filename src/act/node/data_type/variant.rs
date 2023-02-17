use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{node::traits::HasMembers, proclamation::Proclaim},
    keyword,
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub type_: DataType,
}

impl Variant {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Variant", parental_prefix),
        }
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

impl<C> ToTypeAnnotation<C> for Variant {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TokenStream {
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
    ) -> Option<TokenStream> {
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
    ) -> HashMap<String, TokenStream> {
        self.create_member_declarations(keyword_list, self.get_name(parental_prefix.clone()))
    }
}

impl Member {
    fn to_token_stream(&self, keyword_list: &Vec<String>, member_prefix: String) -> TokenStream {
        let member_type_token_stream = match self.type_.clone() {
            DataType::Primitive(_) => {
                if self
                    .type_
                    .to_type_annotation(keyword_list, member_prefix.clone())
                    .to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream =
                        self.type_.to_type_annotation(keyword_list, member_prefix);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_annotation =
                    self.type_.to_type_annotation(keyword_list, member_prefix);
                quote!((#member_type_annotation))
            }
        };
        let member_name = keyword::make_rust_safe(&self.name, keyword_list).to_identifier();
        let rename_attr = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote! {#rename_attr #member_name #member_type_token_stream}
    }
}
