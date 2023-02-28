use proc_macro2::Ident;
use quote::format_ident;

use crate::act::node::DataType;

pub trait ToDataType {
    fn to_data_type(&self) -> DataType;
}

pub trait ToIdent {
    fn to_identifier(&self) -> Ident;
}

impl ToIdent for String {
    fn to_identifier(&self) -> Ident {
        format_ident!("{}", self)
    }
}
