use proc_macro2::TokenStream;
use quote::quote;

use crate::act::node::{
    data_type::type_annotation::{ToTypeAnnotation, TypeAnnotation},
    param::Param,
    proclamation::Proclaim,
    Declaration,
};

pub trait HasParams {
    fn get_params(&self) -> Vec<Param>;

    fn get_name(&self) -> String;

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
    ) -> Option<TypeAnnotation> {
        match self.get_params().get(param_index) {
            Some(param_data_type) => Some(
                param_data_type
                    .to_type_annotation(keyword_list, self.create_param_prefix(param_index)),
            ),
            None => None,
        }
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("{}ParamNum{}", self.get_name(), param_index)
    }

    fn collect_param_inline_declarations(&self, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_params()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, param_type)| {
                let proclamation =
                    param_type.create_proclamation(keyword_list, self.create_param_prefix(index));
                vec![acc, proclamation.flatten()].concat()
            })
    }
}
