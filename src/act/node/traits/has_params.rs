use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::act::{
    self,
    node::{
        data_type::{traits::ToTypeAnnotation, DataType},
        param::Param,
    },
    proclamation::Proclaim,
};

pub trait HasParams {
    fn get_params(&self) -> Vec<Param>;
    fn create_param_prefix(&self, param_index: usize) -> String;

    fn get_param_types(&self) -> Vec<DataType> {
        self.get_params()
            .iter()
            .map(|param| param.type_.clone())
            .collect()
    }

    fn create_parameter_list_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .enumerate()
            .map(|(index, param)| {
                param.to_token_stream(keyword_list, self.create_param_prefix(index))
            })
            .collect();
        quote!(#(#params),*)
    }

    fn create_param_type_annotation(
        &self,
        param_index: usize,
        keyword_list: &Vec<String>,
    ) -> Option<TokenStream> {
        match self.get_param_types().get(param_index) {
            Some(param_data_type) => Some(
                param_data_type
                    .to_type_annotation(keyword_list, self.create_param_prefix(param_index)),
            ),
            None => None,
        }
    }

    fn collect_param_inline_types(
        &self,
        keyword_list: &Vec<String>,
    ) -> HashMap<String, TokenStream> {
        self.get_param_types().iter().enumerate().fold(
            HashMap::new(),
            |acc, (index, param_type)| {
                let proclamation =
                    param_type.create_proclamation(keyword_list, self.create_param_prefix(index));
                act::flatten_proclamation(proclamation, acc)
            },
        )
    }
}
