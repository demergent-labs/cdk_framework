use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{ToIdent, ToTokenStream},
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

impl ToTokenStream<Context> for Vec<TypeParam> {
    fn to_token_stream(&self, _: &Context, _: &str) -> TokenStream {
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

        quote!(#type_params_token_stream #where_clause_token_stream)
    }
}
