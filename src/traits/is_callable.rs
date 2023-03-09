use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{
    node::{Param, ReturnType},
    Declaration,
};

use super::{Declare, HasInlines};

#[derive(Clone)]
pub enum ParamOrReturn {
    ReturnType(ReturnType),
    Param(Param),
}

pub trait IsCallable {
    fn get_params(&self) -> Vec<Param>;
    fn get_return_type(&self) -> Option<ReturnType>;

    fn create_parameter_list_token_stream(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .map(|param| param.to_token_stream(keyword_list, function_name.clone()))
            .collect();
        quote!(#(#params),*)
    }
}

impl<T> HasInlines<ParamOrReturn> for T
where
    T: IsCallable,
{
    fn get_inlines(&self) -> Vec<ParamOrReturn> {
        vec![
            self.get_params()
                .into_iter()
                .map(|param| ParamOrReturn::Param(param))
                .collect::<Vec<_>>(),
            vec![self.get_return_type()]
                .into_iter()
                .filter_map(|x| match x {
                    Some(x) => Some(ParamOrReturn::ReturnType(x)),
                    None => None,
                })
                .collect(),
        ]
        .concat()
    }
}

impl Declare<Vec<String>> for ParamOrReturn {
    fn to_declaration(&self, context: &Vec<String>, function_name: String) -> Option<Declaration> {
        match &self {
            ParamOrReturn::ReturnType(return_type) => {
                return_type.to_declaration(context, function_name)
            }
            ParamOrReturn::Param(param) => param.to_declaration(context, function_name),
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Vec<String>,
        function_name: String,
    ) -> Vec<Declaration> {
        match &self {
            ParamOrReturn::ReturnType(return_type) => {
                return_type.collect_inline_declarations(context, function_name)
            }
            ParamOrReturn::Param(param) => {
                param.collect_inline_declarations(context, function_name)
            }
        }
    }
}