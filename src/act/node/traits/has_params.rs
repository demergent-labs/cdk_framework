use std::collections::HashMap;

use crate::act::declaration::ToDeclaration;

use crate::act::node::{
    canister_method::FnParam,
    data_type::{traits::ToTypeAnnotation, DataType},
};
use proc_macro2::TokenStream;
use quote::quote;

pub trait HasParams {
    fn get_param_types(&self) -> Vec<DataType> {
        self.get_params()
            .iter()
            .map(|param| param.data_type.clone())
            .collect()
    }
    fn get_params(&self) -> Vec<FnParam>;
    fn create_param_prefix(&self, param_index: usize) -> String;
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
        match self.get_params().get(param_index) {
            Some(param) => Some(
                param
                    .data_type
                    .to_type_annotation(keyword_list, self.create_param_prefix(param_index)),
            ),
            None => None,
        }
    }
    fn create_param_declarations(
        &self,
        keyword_list: &Vec<String>,
    ) -> HashMap<String, TokenStream> {
        self.get_param_types().iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (index, param_type)| {
                let declaration =
                    param_type.create_declaration(keyword_list, self.create_param_prefix(index));
                if let Some(identifier) = &declaration.identifier {
                    if let Some(code) = declaration.code {
                        acc.insert(identifier.clone(), code.clone());
                    }
                }
                acc.extend(declaration.children.clone().into_iter());
                acc
            },
        )
    }
}
