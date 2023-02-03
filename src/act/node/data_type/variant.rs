use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::{act::node::full_declaration::ToDeclaration, keyword, traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: Option<String>,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub member_name: String,
    pub member_type: DataType,
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
            .map(|member| member.member_type.clone())
            .collect()
    }

    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String {
        format!("{}VariantMember{}", self.get_name(parental_prefix), index)
    }
}

impl<C> ToTokenStream<C> for Variant {
    fn to_token_stream(&self, context: &C) -> TokenStream {
        self.to_type_annotation(context, "".to_string())
    }
}

impl<C> ToTypeAnnotation<C> for Variant {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TokenStream {
        self.get_name(parental_prefix)
            .to_identifier()
            .to_token_stream()
    }
}

impl ToDeclaration<Vec<String>> for Variant {
    fn create_code(
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
                    self.create_member_prefix(index, parental_prefix.clone()),
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

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        self.create_member_declarations(keyword_list, parental_prefix)
    }
}

impl Member {
    fn to_token_stream(&self, keyword_list: &Vec<String>, parental_prefix: String) -> TokenStream {
        let member_type_token_stream = match self.member_type.clone() {
            DataType::Primitive(_) => {
                if self
                    .member_type
                    .to_type_annotation(keyword_list, parental_prefix.clone())
                    .to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream = self
                        .member_type
                        .to_type_annotation(keyword_list, parental_prefix);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_token_stream = if self.member_type.needs_to_be_boxed() {
                    let ident = self
                        .member_type
                        .to_type_annotation(keyword_list, parental_prefix);
                    quote!(Box<#ident>)
                } else {
                    quote!(self.member_type.to_token_stream())
                };
                quote!((#member_type_token_stream))
            }
        };
        let member_name = keyword::make_rust_safe(&self.member_name, keyword_list).to_identifier();
        let rename = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote! {#rename#member_name#member_type_token_stream}
    }
}
