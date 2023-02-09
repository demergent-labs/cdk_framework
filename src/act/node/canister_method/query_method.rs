use std::collections::HashMap;

use crate::{
    act::{self, declaration::ToDeclaration, node::DataType},
    traits::ToIdent,
};
use proc_macro2::TokenStream;
use quote::quote;

use super::{FnParam, HasParams, HasReturnValue};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct QueryMethod {
    pub body: TokenStream,
    pub params: Vec<FnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub function_guard_name: Option<String>,
}

impl QueryMethod {
    fn generate_kybra_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];
        if self.is_async {
            args.push(quote! {composite = true});
        };
        if self.is_manual {
            args.push(quote! {manual_reply = true});
        };
        if let Some(guard_function) = &self.function_guard_name {
            args.push(quote! {guard = #guard_function});
        };

        quote!(#(#args),*)
    }

    fn generate_not_kybra_macro_args(&self) -> TokenStream {
        if self.is_async {
            quote! {composite = true, manual_reply = true}
        } else if self.is_manual {
            quote! {manual_reply = true}
        } else {
            quote! {}
        }
    }

    fn generate_function(&self, keyword_list: &Vec<String>, _: String) -> TokenStream {
        let function_name = self.name.to_identifier();
        let params = self.create_parameter_list_token_stream(keyword_list);
        let function_body = &self.body;

        let return_type_token = self.create_return_type_annotation(keyword_list);
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

impl HasParams for QueryMethod {
    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("{}ParamNum{}", self.name, param_index)
    }

    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }
}

impl HasReturnValue for QueryMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }

    fn create_return_type_prefix(&self) -> String {
        format!("{}ReturnType", self.name)
    }
}

impl ToDeclaration<Vec<String>> for QueryMethod {
    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        let param_declarations = self.create_param_declarations(keyword_list);
        let result_declarations = self.create_return_type_declarations(keyword_list);
        act::combine_maps(param_declarations, result_declarations)
    }

    fn create_code(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        let prefix = format!("{}{}", parental_prefix, self.name);
        let function_signature = self.generate_function(keyword_list, prefix);
        let macro_args = if self.cdk_name == "kybra" {
            self.generate_kybra_macro_args()
        } else {
            self.generate_not_kybra_macro_args()
        };
        Some(quote! {
            #[ic_cdk_macros::query(#macro_args)]
            #[candid::candid_method(query)]
            #function_signature
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }
}
