use quote::quote;

use super::type_annotation::{ToTypeAnnotation, TypeAnnotation};
use crate::act::node::{proclamation::Proclaim, Declaration};

#[derive(Clone, Debug)]
pub enum Primitive {
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

impl<C> ToTypeAnnotation<C> for Primitive {
    fn to_type_annotation(&self, _: &C, _: String) -> TypeAnnotation {
        match self {
            Primitive::Bool => quote!(bool),
            Primitive::Blob => quote!(Vec<u8>),
            Primitive::Empty => quote!(candid::Empty),
            Primitive::Float32 => quote!(f32),
            Primitive::Float64 => quote!(f64),
            Primitive::Int => quote!(candid::Int),
            Primitive::Int8 => quote!(i8),
            Primitive::Int16 => quote!(i16),
            Primitive::Int32 => quote!(i32),
            Primitive::Int64 => quote!(i64),
            Primitive::Nat => quote!(candid::Nat),
            Primitive::Nat8 => quote!(u8),
            Primitive::Nat16 => quote!(u16),
            Primitive::Nat32 => quote!(u32),
            Primitive::Nat64 => quote!(u64),
            Primitive::Null => quote! {(())},
            Primitive::Principal => quote!(candid::Principal),
            Primitive::Reserved => quote!(candid::Reserved),
            Primitive::String => quote!(String),
            Primitive::Void => quote! {()},
        }
    }
}

impl Proclaim<Vec<String>> for Primitive {
    fn create_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<Declaration> {
        vec![]
    }
}
