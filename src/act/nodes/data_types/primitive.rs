use proc_macro2::TokenStream;
use quote::quote;

use super::{traits::ToIdent, ActDataType, LiteralOrTypeAlias};
use crate::{act::actable::ToActDataType, ToTokenStream};

#[derive(Clone, Debug)]
pub struct ActPrimitive {
    pub act_type: LiteralOrTypeAlias<ActPrimitiveLit, ActPrimitiveTypeAlias>,
}

#[derive(Clone, Debug)]
pub enum ActPrimitiveLit {
    Bool,
    Blob,
    Empty,
    Float32,
    Float64,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Nat,
    Nat8,
    Nat16,
    Nat32,
    Nat64,
    Null,
    Principal,
    Reserved,
    String,
    Void,
}

impl ToActDataType for ActPrimitiveLit {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> ActDataType {
        ActDataType::Primitive(ActPrimitive {
            act_type: match alias_name {
                None => LiteralOrTypeAlias::Literal(self.clone()),
                Some(name) => LiteralOrTypeAlias::TypeAlias(ActPrimitiveTypeAlias {
                    name: name.clone().clone(),
                    aliased_type: self.clone(),
                }),
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct ActPrimitiveTypeAlias {
    pub name: String,
    pub aliased_type: ActPrimitiveLit,
}

impl<C> ToTokenStream<C> for ActPrimitiveTypeAlias {
    fn to_token_stream(&self, context: C) -> TokenStream {
        let name = self.name.to_identifier();
        let alias = self.aliased_type.to_token_stream(context);
        quote!(type #name = #alias;)
    }
}

impl<C> ToTokenStream<C> for ActPrimitiveLit {
    fn to_token_stream(&self, _: C) -> TokenStream {
        match self {
            ActPrimitiveLit::Bool => quote!(bool),
            ActPrimitiveLit::Blob => quote!(Vec<u8>),
            ActPrimitiveLit::Empty => quote!(candid::Empty),
            ActPrimitiveLit::Float32 => quote!(f32),
            ActPrimitiveLit::Float64 => quote!(f64),
            ActPrimitiveLit::Int => quote!(candid::Int),
            ActPrimitiveLit::Int8 => quote!(i8),
            ActPrimitiveLit::Int16 => quote!(i16),
            ActPrimitiveLit::Int32 => quote!(i32),
            ActPrimitiveLit::Int64 => quote!(i64),
            ActPrimitiveLit::Nat => quote!(candid::Nat),
            ActPrimitiveLit::Nat8 => quote!(u8),
            ActPrimitiveLit::Nat16 => quote!(u16),
            ActPrimitiveLit::Nat32 => quote!(u32),
            ActPrimitiveLit::Nat64 => quote!(u64),
            ActPrimitiveLit::Null => quote! {(())},
            ActPrimitiveLit::Principal => quote!(candid::Principal),
            ActPrimitiveLit::Reserved => quote!(candid::Reserved),
            ActPrimitiveLit::String => quote!(String),
            ActPrimitiveLit::Void => quote! {()},
        }
    }
}

impl ToTokenStream<&Vec<String>> for ActPrimitive {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.act_type.to_token_stream(context)
    }
}
