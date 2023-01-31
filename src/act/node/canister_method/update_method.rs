use crate::{act::node::DataType, traits::ToIdent, ToTokenStream, ToTokenStreams};
use proc_macro2::TokenStream;
use quote::quote;

use super::{ActFnParam, HasParams, HasReturnValue};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct UpdateMethod {
    pub body: TokenStream,
    pub params: Vec<ActFnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub function_guard_name: Option<String>,
}

impl UpdateMethod {
    // TODO this is exactly the same as the query version. Is that an issue we want to resolve?
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

impl ToTokenStream<&Vec<String>> for UpdateMethod {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_signature = self.generate_function(keyword_list);

        let manual_reply_arg = if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
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

impl HasParams for UpdateMethod {
    fn get_param_types(&self) -> Vec<DataType> {
        self.params
            .iter()
            .map(|param| param.data_type.clone())
            .collect()
    }
}

impl HasReturnValue for UpdateMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }
}

// impl ToTokenStream<&Vec<String>> for ActCanisterMethod {
//     fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
//         match self {
//             ActCanisterMethod::QueryMethod(query_method) => {}
//             ActCanisterMethod::UpdateMethod(update_method) => {
//         }
//     }
// }

// impl ActCanisterMethod {
//     // pub fn get_name(&self) -> String {
//     //     match self {
//     //         ActCanisterMethod::QueryMethod(canister_method) => &canister_method.name,
//     //         ActCanisterMethod::UpdateMethod(canister_method) => &canister_method.name,
//     //     }
//     //     .clone()
//     // }

//     // pub fn get_return_type(&self) -> ActDataType {
//     //     match self {
//     //         ActCanisterMethod::QueryMethod(canister_method) => &canister_method.return_type,
//     //         ActCanisterMethod::UpdateMethod(canister_method) => &canister_method.return_type,
//     //     }
//     //     .clone()
//     // }

//     // pub fn get_param_types(&self) -> Vec<ActDataType> {
//     //     match self {
//     //         ActCanisterMethod::QueryMethod(query) => &query.params,
//     //         ActCanisterMethod::UpdateMethod(update) => &update.params,
//     //     }
//     //     .iter()
//     //     .map(|param| param.data_type.clone())
//     //     .collect()
//     // }

//     // pub fn is_manual(&self) -> bool {
//     //     match self {
//     //         ActCanisterMethod::QueryMethod(canister_method) => canister_method.is_manual,
//     //         ActCanisterMethod::UpdateMethod(canister_method) => canister_method.is_manual,
//     //     }
//     // }

//     // pub fn is_async(&self) -> bool {
//     //     match self {
//     //         ActCanisterMethod::QueryMethod(canister_method) => canister_method.is_async,
//     //         ActCanisterMethod::UpdateMethod(canister_method) => canister_method.is_async,
//     //     }
//     // }
// }
