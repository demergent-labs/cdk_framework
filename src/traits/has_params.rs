use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::param::Param, Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

pub trait HasParams {
    fn get_params(&self) -> Vec<Param>;

    fn get_inline_prefix(&self) -> String;

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
            Some(candid_type) => Some(
                candid_type.to_type_annotation(keyword_list, self.create_param_prefix(param_index)),
            ),
            None => None,
        }
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!(
            "{name}ParamNum{param_index}",
            name = self.get_inline_prefix()
        )
    }

    fn collect_param_inline_declarations(&self, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_params()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, param_type)| {
                let declarations =
                    param_type.flatten(keyword_list, self.create_param_prefix(index));
                vec![acc, declarations].concat()
            })
    }
}
