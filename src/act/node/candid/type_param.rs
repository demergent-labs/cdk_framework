use std::ops::Deref;

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
        vec![]
    }
}

#[derive(Clone, Debug)]
pub struct TypeParams(pub Vec<TypeParam>);

impl Deref for TypeParams {
    type Target = Vec<TypeParam>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<TypeParam>> for TypeParams {
    fn from(vec: Vec<TypeParam>) -> Self {
        TypeParams(vec)
    }
}

impl TypeParams {
    pub fn get_type_params_token_stream(&self) -> TokenStream {
        let type_param_token_streams: Vec<TokenStream> = self
            .iter()
            .map(|type_param| {
                let name = type_param.name.to_ident();
                let try_into_vm_value_trait_bound = &type_param.try_into_vm_value_trait_bound;

                quote!(#name: #try_into_vm_value_trait_bound)
            })
            .collect();

        let type_params_token_stream = if type_param_token_streams.len() != 0 {
            quote!(<#(#type_param_token_streams),*>)
        } else {
            quote!()
        };

        type_params_token_stream
    }

    pub fn get_where_clause_token_stream(&self) -> TokenStream {
        let where_clause_token_streams: Vec<TokenStream> = self
            .iter()
            .map(|type_param| {
                let try_from_vm_value_trait_bound =
                    (&type_param.try_from_vm_value_trait_bound)(type_param.name.clone());

                try_from_vm_value_trait_bound
            })
            .collect();

        let where_clause_token_stream = if where_clause_token_streams.len() != 0 {
            quote!(where #(#where_clause_token_streams),*)
        } else {
            quote!()
        };

        where_clause_token_stream
    }
}
