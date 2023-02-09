use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::traits::{HasParams, HasReturnValue},
    traits::ToIdent,
};

pub mod query_method;
pub mod update_method;

pub trait PublicCanisterMethod: HasParams + HasReturnValue {
    fn get_name(&self) -> String;
    fn get_body(&self) -> TokenStream;
    fn get_cdk_name(&self) -> String;
    fn is_manual(&self) -> bool;
    fn is_async(&self) -> bool;

    fn generate_function_declaration(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_name = self.get_name().to_identifier();
        let params = self.create_parameter_list_token_stream(keyword_list);

        let function_body = self.get_body();

        let return_type_token = self.create_return_type_annotation(keyword_list);
        let wrapped_return_type =
            if self.is_manual() || (self.is_async() && self.get_cdk_name() != "kybra") {
                quote! {
                    ic_cdk::api::call::ManualReply<#return_type_token>
                }
            } else {
                return_type_token
            };

        quote! {
            async fn #function_name(#params) -> #wrapped_return_type {
                #function_body
            }
        }
    }
}
