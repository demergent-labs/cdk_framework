use proc_macro2::TokenStream;

use super::DataType;

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;
}

pub trait ToTypeAnnotation<C> {
    fn to_type_annotation(&self, context: &C) -> TokenStream;
}
