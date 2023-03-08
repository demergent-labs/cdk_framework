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
            .map(|param| {
                param.to_token_stream(keyword_list, self.create_param_prefix(param.name.clone()))
            })
            .collect();
        quote!(#(#params),*)
    }

    fn create_param_type_annotation(
        &self,
        param: &Param,
        keyword_list: &Vec<String>,
    ) -> TypeAnnotation {
        param.to_type_annotation(keyword_list, self.create_param_prefix(param.name.clone()))
    }

    fn create_param_prefix(&self, param_name: String) -> String {
        format!(
            "{prefix}ParamNum{param_name}",
            prefix = self.get_inline_prefix()
        )
    }

    fn collect_param_inline_declarations(&self, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_params().iter().fold(vec![], |acc, param| {
            let declarations =
                param.flatten(keyword_list, self.create_param_prefix(param.name.clone()));
            vec![acc, declarations].concat()
        })
    }
}
