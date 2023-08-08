use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::Context,
    traits::{ToTokenStream, ToTypeAnnotation},
};

use super::CandidType;

#[derive(Clone, Debug)]
pub struct TypeArg(pub CandidType);

impl TypeArg {
    pub fn get_inline_name(&self, inline_name: &str, index: usize) -> String {
        format!("{inline_name}TypeArg{index}")
    }
}

impl Deref for TypeArg {
    type Target = CandidType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokenStream<Context> for Vec<TypeArg> {
    fn to_token_stream(&self, context: &Context, inline_name: &str) -> proc_macro2::TokenStream {
        let type_argument_token_streams: Vec<TokenStream> = self
            .iter()
            .enumerate()
            .map(|(index, type_argument)| {
                type_argument.to_type_annotation(
                    context,
                    type_argument.get_inline_name(&inline_name, index),
                    &None,
                )
            })
            .collect();
        let type_arguments_token_stream = if type_argument_token_streams.len() != 0 {
            quote!(<#(#type_argument_token_streams),*>)
        } else {
            quote!()
        };

        type_arguments_token_stream
    }
}
