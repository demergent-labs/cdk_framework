use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{CandidType, Param, ReturnType},
    traits::{HasParams, HasReturnValue, ToIdent, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct QueryOrUpdateDefinition {
    pub body: TokenStream,
    pub params: Vec<Param>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: CandidType,
    pub cdk_name: String,
    pub guard_function_name: Option<String>,
}

impl QueryOrUpdateDefinition {
    pub fn generate_function_body(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_name = self.name.to_ident();
        let params = self.create_parameter_list_token_stream(&self.name, keyword_list);

        let function_body = &self.body;

        let return_type_token = ReturnType::new(self.return_type.clone())
            .to_type_annotation(keyword_list, self.name.clone());

        let wrapped_return_type = if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
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

impl HasParams for QueryOrUpdateDefinition {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}

impl HasReturnValue for QueryOrUpdateDefinition {
    fn get_return_type(&self) -> ReturnType {
        ReturnType::new(self.return_type.clone())
    }
}
