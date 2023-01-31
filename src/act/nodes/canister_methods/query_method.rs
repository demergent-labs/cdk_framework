use crate::{
    act::nodes::{data_types::traits::ToIdent, ActDataType},
    ToTokenStream, ToTokenStreams,
};
use proc_macro2::TokenStream;
use quote::quote;

use super::{ActFnParam, HasParams, HasReturnValue};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct QueryMethod {
    pub body: TokenStream,
    pub params: Vec<ActFnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: ActDataType,
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

    fn generate_function(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_name = self.name.to_identifier();
        let params = self.params.to_token_streams(keyword_list);

        let function_body = &self.body;

        let return_type_token = self.return_type.to_token_stream(keyword_list);
        let wrapped_return_type = if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
            quote! {
                ic_cdk::api::call::ManualReply<#return_type_token>
            }
        } else {
            return_type_token
        };

        quote! {
            async fn #function_name(#(#params),*) -> #wrapped_return_type {
                #function_body
            }
        }
    }
}

impl ToTokenStream<&Vec<String>> for QueryMethod {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_signature = self.generate_function(keyword_list);
        let macro_args = if self.cdk_name == "kybra" {
            self.generate_kybra_macro_args()
        } else {
            self.generate_not_kybra_macro_args()
        };
        quote! {
            #[ic_cdk_macros::query(#macro_args)]
            #[candid::candid_method(query)]
            #function_signature
        }
    }
}

impl HasParams for QueryMethod {
    fn get_param_types(&self) -> Vec<ActDataType> {
        self.params
            .iter()
            .map(|param| param.data_type.clone())
            .collect()
    }
}

impl HasReturnValue for QueryMethod {
    fn get_return_type(&self) -> ActDataType {
        self.return_type.clone()
    }
}
