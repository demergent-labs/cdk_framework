use super::{traits::HasMembers, DataType};
use crate::{keyword, traits::ToIdent, ToDeclarationTokenStream, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub member_name: String,
    pub member_type: DataType,
}

impl HasMembers for Variant {
    fn get_members(&self) -> Vec<DataType> {
        self.members
            .iter()
            .map(|member| member.member_type.clone())
            .collect()
    }
}

impl<C> ToTokenStream<C> for Variant {
    fn to_token_stream(&self, _: C) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToDeclarationTokenStream<&Vec<String>> for Variant {
    fn to_declaration(&self, keyword_list: &Vec<String>, _: String) -> TokenStream {
        let type_ident = self.name.to_identifier();
        let member_token_streams: Vec<TokenStream> = self
            .members
            .iter()
            .map(|member| member.to_token_stream(keyword_list))
            .collect();
        quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            enum #type_ident {
                #(#member_token_streams),*
            }
        )
    }
}

impl ToTokenStream<&Vec<String>> for Member {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let member_type_token_stream = match self.member_type.clone() {
            DataType::Primitive(_) => {
                if self.member_type.to_token_stream(keyword_list).to_string()
                    == quote!((())).to_string()
                {
                    quote!()
                } else {
                    let member_type_token_stream = self.member_type.to_token_stream(keyword_list);
                    quote!((#member_type_token_stream))
                }
            }
            _ => {
                let member_type_token_stream = if self.member_type.needs_to_be_boxed() {
                    let ident = self.member_type.to_token_stream(keyword_list);
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
