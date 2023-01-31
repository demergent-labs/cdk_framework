pub use act_data_type::{build_inline_type_acts, deduplicate, ActDataType};
pub use arrays::{ActArray, ActArrayLiteral, ActArrayTypeAlias};
pub use funcs::{generate_func_arg_token, Func};
pub use option::{ActOption, ActOptionLiteral, ActOptionTypeAlias};
pub use primitives::{ActPrimitive, ActPrimitiveLit, ActPrimitiveTypeAlias};
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
pub use record::{ActRecordMember, Record};
pub use tuple::{ActTupleElem, Tuple};
pub use type_ref::{ActTypeRef, ActTypeRefLit, ActTypeRefTypeAlias};
pub use variants::{ActVariantMember, Variant};

use crate::ToTokenStream;

pub mod act_data_type;
pub mod arrays;
pub mod funcs;
pub mod option;
pub mod primitives;
pub mod record;
pub mod tuple;
pub mod type_ref;
pub mod variants;

pub trait ToIdent {
    fn to_identifier(&self) -> Ident;
}

impl ToIdent for String {
    fn to_identifier(&self) -> Ident {
        format_ident!("{}", self)
    }
}

pub trait TypeAliasize<T> {
    fn as_type_alias(&self) -> T;
}

pub trait Literally {
    fn is_literal(&self) -> bool;
}

pub trait HasMembers {
    fn get_members(&self) -> Vec<ActDataType>;
}

#[derive(Clone, Debug)]
pub enum LiteralOrTypeAlias<L, T> {
    Literal(L),
    TypeAlias(T),
}

impl<L: ToTokenStream<C>, T: ToTokenStream<C>, C> ToTokenStream<C> for LiteralOrTypeAlias<L, T> {
    fn to_token_stream(&self, context: C) -> TokenStream {
        match self {
            LiteralOrTypeAlias::Literal(literal) => literal.to_token_stream(context),
            LiteralOrTypeAlias::TypeAlias(type_alias) => type_alias.to_token_stream(context),
        }
    }
}

impl<L, T> Literally for LiteralOrTypeAlias<L, T> {
    fn is_literal(&self) -> bool {
        match self {
            LiteralOrTypeAlias::Literal(_) => true,
            LiteralOrTypeAlias::TypeAlias(_) => false,
        }
    }
}
