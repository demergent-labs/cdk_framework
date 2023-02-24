use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::act::node::{
    traits::{HasParams, HasReturnValue},
    DataType,
};

pub trait QueryOrUpdateMethod: HasParams + HasReturnValue {
    fn get_name(&self) -> String;
    fn get_body(&self) -> TokenStream;
    fn get_cdk_name(&self) -> String;
    fn is_manual(&self) -> bool;
    fn is_async(&self) -> bool;

    fn generate_function_body(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_name = self.get_name().to_identifier();
        let params = self.create_parameter_list_token_stream(keyword_list, &self.get_name());

        let function_body = self.get_body();

        let return_type_token = self.create_return_type_annotation(keyword_list, &self.get_name());
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

pub trait ToDataType {
    fn to_data_type(&self) -> DataType;
}

pub trait ToIdent {
    fn to_identifier(&self) -> Ident;
}

impl ToIdent for String {
    fn to_identifier(&self) -> Ident {
        format_ident!("{}", self)
    }
}
