use proc_macro2::TokenStream;
use quote::quote;

use crate::act::node::Param;

use super::HasDeclarableTypes;

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
            .map(|param| param.to_token_stream(keyword_list, function_name.clone()))
            .collect();
        quote!(#(#params),*)
    }
}

impl<T> HasDeclarableTypes<Param> for T
where
    T: HasParams,
{
    fn get_declarable_items(&self) -> Vec<Param> {
        self.get_params()
    }
}
