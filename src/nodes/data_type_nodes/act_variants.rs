use super::{ActDataType, HasMembers, LiteralOrTypeAlias, ToIdent, TypeAliasize};
use crate::{keyword, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct ActVariant {
    pub act_type: LiteralOrTypeAlias<VariantLiteral, VariantTypeAlias>,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub members: Vec<ActVariantMember>,
}

#[derive(Clone, Debug)]
pub struct VariantLiteral {
    pub variant: Variant,
}

#[derive(Clone, Debug)]
pub struct VariantTypeAlias {
    pub variant: Variant,
}

#[derive(Clone, Debug)]
pub struct ActVariantMember {
    pub member_name: String,
    pub member_type: ActDataType,
}

impl TypeAliasize<ActVariant> for ActVariant {
    fn as_type_alias(&self) -> ActVariant {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => ActVariant {
                act_type: LiteralOrTypeAlias::TypeAlias(VariantTypeAlias {
                    variant: literal.variant.clone(),
                }),
            },
            LiteralOrTypeAlias::TypeAlias(_) => self.clone(),
        }
    }
}

impl HasMembers for ActVariant {
    fn get_members(&self) -> Vec<ActDataType> {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => &literal.variant,
            LiteralOrTypeAlias::TypeAlias(type_alias) => &type_alias.variant,
        }
        .members
        .iter()
        .map(|member| member.member_type.clone())
        .collect()
    }
}

impl<C> ToTokenStream<C> for VariantLiteral {
    fn to_token_stream(&self, _: C) -> TokenStream {
        self.variant.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<&Vec<String>> for VariantTypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let type_ident = self.variant.name.to_identifier();
        let member_token_streams: Vec<TokenStream> = self
            .variant
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

impl ToTokenStream<&Vec<String>> for ActVariantMember {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let member_type_token_stream = match self.member_type.clone() {
            ActDataType::Primitive(_) => {
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
