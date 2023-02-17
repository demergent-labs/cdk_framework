use proc_macro2::TokenStream;
use quote::quote;

use crate::act::node::{
    data_type::{
        type_annotation::{ToTypeAnnotation, TypeAnnotation},
        DataType,
    },
    param::Param,
    proclamation::Proclaim,
    Declaration,
};

pub trait HasParams {
    fn get_params(&self) -> Vec<Param>;

    fn get_param_types(&self) -> Vec<DataType> {
        self.get_params()
            .iter()
            .map(|param| param.type_.clone())
            .collect()
    }

    fn create_parameter_list_token_stream(
        &self,
        keyword_list: &Vec<String>,
        name: &String,
    ) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .enumerate()
            .map(|(index, param)| {
                param.to_token_stream(keyword_list, self.create_param_prefix(index, name))
            })
            .collect();
        quote!(#(#params),*)
    }

    fn create_param_type_annotation(
        &self,
        param_index: usize,
        keyword_list: &Vec<String>,
        name: &String,
    ) -> Option<TypeAnnotation> {
        match self.get_param_types().get(param_index) {
            Some(param_data_type) => Some(
                param_data_type
                    .to_type_annotation(keyword_list, self.create_param_prefix(param_index, name)),
            ),
            None => None,
        }
    }

    fn create_param_prefix(&self, param_index: usize, parental_prefix: &String) -> String {
        format!("{}ParamNum{}", parental_prefix, param_index)
    }

    fn collect_param_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: &String,
    ) -> Vec<Declaration> {
        self.get_param_types()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, param_type)| {
                let proclamation = param_type
                    .create_proclamation(keyword_list, self.create_param_prefix(index, name));
                vec![acc, proclamation.flatten()].concat()
            })
    }
}
