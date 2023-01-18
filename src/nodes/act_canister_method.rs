use crate::{nodes::ActFnParam, ActDataType, ToTokenStream, ToTokenStreams};
use proc_macro2::TokenStream;
use quote::quote;

use super::data_type_nodes::ToIdent;

#[derive(Debug, Clone)]
pub enum ActCanisterMethod {
    QueryMethod(CanisterMethod),
    UpdateMethod(CanisterMethod),
}

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct CanisterMethod {
    pub body: TokenStream,
    pub params: Vec<ActFnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: ActDataType,
    pub cdk_name: String,
}

pub fn get_all_types_from_canister_method_acts(
    canister_methods: &Vec<ActCanisterMethod>,
) -> Vec<ActDataType> {
    canister_methods
        .iter()
        .fold(vec![], |acc, canister_method| {
            let inline_types = canister_method.get_all_types();
            vec![acc, inline_types].concat()
        })
}

fn generate_kybra_version(query_method: &CanisterMethod) -> TokenStream {
    let composite_arg = if query_method.is_async {
        quote! {composite = true}
    } else {
        quote!()
    };
    let manual_reply_arg = if query_method.is_manual {
        quote! {manual_reply = true}
    } else {
        quote! {}
    };

    // If both are true then we need a comma separated list.
    // Otherwise we can just join them together because they are
    // both empty quotes or one is an empty quote and the other
    // needs to go in the parenthesis
    if query_method.is_async && query_method.is_manual {
        quote! {#composite_arg, #manual_reply_arg}
    } else {
        quote! {#composite_arg#manual_reply_arg}
    }
}

fn generate_not_kybra_version(query_method: &CanisterMethod) -> TokenStream {
    if query_method.is_async {
        quote! {composite = true, manual_reply = true}
    } else if query_method.is_manual {
        quote! {manual_reply = true}
    } else {
        quote! {}
    }
}

impl ToTokenStream<&Vec<String>> for ActCanisterMethod {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        match self {
            ActCanisterMethod::QueryMethod(query_method) => {
                let function_signature = generate_function(query_method, keyword_list);
                let macro_args = if query_method.cdk_name == "kybra" {
                    generate_kybra_version(query_method)
                } else {
                    generate_not_kybra_version(query_method)
                };
                quote! {
                    #[ic_cdk_macros::query(#macro_args)]
                    #[candid::candid_method(query)]
                    #function_signature
                }
            }
            ActCanisterMethod::UpdateMethod(update_method) => {
                let function_signature = generate_function(update_method, keyword_list);

                let manual_reply_arg = if update_method.is_manual
                    || (update_method.is_async && update_method.cdk_name != "kybra")
                {
                    quote! {(manual_reply = true)}
                } else {
                    quote! {}
                };

                quote! {
                    #[ic_cdk_macros::update#manual_reply_arg]
                    #[candid::candid_method(update)]
                    #function_signature
                }
            }
        }
    }
}

impl ActCanisterMethod {
    pub fn get_all_types(&self) -> Vec<ActDataType> {
        vec![self.get_param_types(), vec![self.get_return_type()]].concat()
    }

    pub fn get_name(&self) -> String {
        match self {
            ActCanisterMethod::QueryMethod(canister_method) => &canister_method.name,
            ActCanisterMethod::UpdateMethod(canister_method) => &canister_method.name,
        }
        .clone()
    }

    pub fn get_return_type(&self) -> ActDataType {
        match self {
            ActCanisterMethod::QueryMethod(canister_method) => &canister_method.return_type,
            ActCanisterMethod::UpdateMethod(canister_method) => &canister_method.return_type,
        }
        .clone()
    }

    pub fn get_param_types(&self) -> Vec<ActDataType> {
        match self {
            ActCanisterMethod::QueryMethod(query) => &query.params,
            ActCanisterMethod::UpdateMethod(update) => &update.params,
        }
        .iter()
        .map(|param| param.data_type.clone())
        .collect()
    }

    pub fn is_manual(&self) -> bool {
        match self {
            ActCanisterMethod::QueryMethod(canister_method) => canister_method.is_manual,
            ActCanisterMethod::UpdateMethod(canister_method) => canister_method.is_manual,
        }
    }

    pub fn is_async(&self) -> bool {
        match self {
            ActCanisterMethod::QueryMethod(canister_method) => canister_method.is_async,
            ActCanisterMethod::UpdateMethod(canister_method) => canister_method.is_async,
        }
    }
}

fn generate_function(canister_method: &CanisterMethod, keyword_list: &Vec<String>) -> TokenStream {
    let function_name = canister_method.name.to_identifier();
    let params = canister_method.params.to_token_streams(keyword_list);

    let function_body = &canister_method.body;

    let return_type_token = canister_method.return_type.to_token_stream(keyword_list);
    let wrapped_return_type = if canister_method.is_manual
        || (canister_method.is_async && canister_method.cdk_name != "kybra")
    {
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
