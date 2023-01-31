use super::{
    traits::{HasMembers, ToIdent, TypeAliasize},
    ActDataType, LiteralOrTypeAlias,
};
use crate::{keyword, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct ActRecord {
    pub act_type: LiteralOrTypeAlias<RecordLiteral, RecordTypeAlias>,
}

#[derive(Clone, Debug)]
pub struct Record {
    pub name: String,
    pub members: Vec<ActRecordMember>,
}

#[derive(Clone, Debug)]
pub struct RecordLiteral {
    pub record: Record,
}

#[derive(Clone, Debug)]
pub struct RecordTypeAlias {
    pub record: Record,
}

#[derive(Clone, Debug)]
pub struct ActRecordMember {
    pub member_name: String,
    pub member_type: ActDataType,
}

impl TypeAliasize<ActRecord> for ActRecord {
    fn as_type_alias(&self) -> ActRecord {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => ActRecord {
                act_type: LiteralOrTypeAlias::TypeAlias(RecordTypeAlias {
                    record: literal.record.clone(),
                }),
            },
            LiteralOrTypeAlias::TypeAlias(_) => self.clone(),
        }
    }
}

// TODO see if I can do this
impl TypeAliasize<RecordTypeAlias> for RecordLiteral {
    fn as_type_alias(&self) -> RecordTypeAlias {
        RecordTypeAlias {
            record: self.record.clone(),
        }
    }
}

impl HasMembers for ActRecord {
    fn get_members(&self) -> Vec<ActDataType> {
        self.get_member_types()
    }
}

impl ActRecord {
    pub fn get_member_types(&self) -> Vec<ActDataType> {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => &literal.record,
            LiteralOrTypeAlias::TypeAlias(type_alias) => &type_alias.record,
        }
        .members
        .iter()
        .map(|member| member.member_type.clone())
        .collect()
    }
}

impl<C> ToTokenStream<C> for RecordLiteral {
    fn to_token_stream(&self, _: C) -> TokenStream {
        self.record.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<&Vec<String>> for RecordTypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let type_ident = self.record.name.to_identifier();
        let member_token_streams: Vec<TokenStream> = self
            .record
            .members
            .iter()
            .map(|member| member.to_token_stream(keyword_list))
            .collect();
        quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident {
                #(#member_token_streams),*
            }
        )
    }
}

impl ToTokenStream<&Vec<String>> for ActRecordMember {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let member_type_token_stream = if self.member_type.needs_to_be_boxed() {
            let ident = self.member_type.to_token_stream(keyword_list);
            quote!(Box<#ident>)
        } else {
            quote!(self.member_type.to_token_stream())
        };
        let member_name = keyword::make_rust_safe(&self.member_name, keyword_list).to_identifier();
        let rename = keyword::generate_rename_attribute(&member_name, keyword_list);
        quote!(#rename#member_name: #member_type_token_stream)
    }
}

impl ToTokenStream<&Vec<String>> for ActRecord {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.act_type.to_token_stream(context)
    }
}
