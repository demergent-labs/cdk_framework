use proc_macro2::TokenStream;

use super::{DataType, LiteralOrTypeAlias};
use crate::ToTokenStream;

pub trait TypeAliasize<T> {
    fn as_type_alias(&self) -> T;
}

pub trait Literally {
    fn is_literal(&self) -> bool;
}

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;
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
