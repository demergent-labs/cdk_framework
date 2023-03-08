use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::param::Param, Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

pub trait HasParams {
    fn get_params(&self) -> Vec<Param>;

    fn create_parameter_list_token_stream(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .map(|param| {
                param.to_token_stream(
                    keyword_list,
                    self.create_param_prefix(&param.name, function_name),
                )
            })
            .collect();
        quote!(#(#params),*)
    }

    fn create_param_type_annotation(
        &self,
        param: &Param,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> TypeAnnotation {
        param.to_type_annotation(
            keyword_list,
            self.create_param_prefix(&param.name, function_name),
        )
    }

    fn create_param_prefix(&self, param_name: &String, function_name: &String) -> String {
        format!("{function_name}ParamNum{param_name}",)
    }

    fn collect_param_inline_declarations(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> Vec<Declaration> {
        self.get_params().iter().fold(vec![], |acc, param| {
            let declarations = param.flatten(
                keyword_list,
                self.create_param_prefix(&param.name, function_name),
            );
            vec![acc, declarations].concat()
        })
    }
}
