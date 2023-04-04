use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::ToIdent,
};

use super::CandidType;

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
    pub type_arguments: Vec<CandidType>,
}

impl ToTypeAnnotation<Context> for TypeRef {
    fn to_type_annotation(&self, context: &Context, inline_name: String) -> TypeAnnotation {
        // TODO use the keyword list to make the identifier rust safe
        let name = self.name.to_ident().to_token_stream();

        let type_argument_token_streams: Vec<TokenStream> = self
            .type_arguments
            .iter()
            .enumerate()
            .map(|(index, type_argument)| {
                type_argument
                    .to_type_annotation(context, get_type_arg_inline_name(&inline_name, index))
            })
            .collect();
        let type_arguments_token_stream = if type_argument_token_streams.len() != 0 {
            quote!(<#(#type_argument_token_streams),*>)
        } else {
            quote!()
        };

        quote!(#name #type_arguments_token_stream)
    }
}

impl Declare<Context> for TypeRef {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.type_arguments
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, item)| {
                vec![
                    acc,
                    item.flatten(context, get_type_arg_inline_name(&inline_name, index)),
                ]
                .concat()
            })
    }
}

fn get_type_arg_inline_name(inline_name: &str, index: usize) -> String {
    format!("{inline_name}TypeArg{index}")
}
