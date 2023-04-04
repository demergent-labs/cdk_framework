use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct TypeParam {
    pub name: String,
    pub try_into_vm_value_trait_bound: TokenStream,
    pub try_from_vm_value_trait_bound: fn(String) -> TokenStream,
}

impl ToTypeAnnotation<Context> for TypeParam {
    fn to_type_annotation(&self, _: &Context, _: String) -> TypeAnnotation {
        let ident = self.name.to_ident();
        quote!(#ident)
    }
}

impl Declare<Context> for TypeParam {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![] // TODO do we need this?
    }
}
