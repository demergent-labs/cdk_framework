use proc_macro2::TokenStream;
use quote::quote;

use crate::act::{node::param::Param, Declaration, Declare};

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
                    param.create_type_prefix(function_name.clone()),
                )
            })
            .collect();
        quote!(#(#params),*)
    }

    fn collect_param_inline_declarations(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> Vec<Declaration> {
        self.get_params().iter().fold(vec![], |acc, param| {
            let declarations = param.flatten(
                keyword_list,
                param.create_type_prefix(function_name.clone()),
            );
            vec![acc, declarations].concat()
        })
    }
}
