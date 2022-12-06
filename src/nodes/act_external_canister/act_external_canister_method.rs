use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{nodes::ActFnParam, ActDataType, ToTokenStream};

#[derive(Clone, Debug)]
pub struct ActExternalCanisterMethod {
    pub name: String,
    pub params: Vec<ActFnParam>,
    pub return_type: ActDataType,
}

pub struct ActEcmContext<'a> {
    pub canister_name: String,
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToTokenStream<ActEcmContext<'_>> for ActExternalCanisterMethod {
    fn to_token_stream(&self, context: ActEcmContext) -> TokenStream {
        let call_function = self.generate_function("call", &context);
        let call_with_payment_function = self.generate_function("call_with_payment", &context);
        let call_with_payment128_function =
            self.generate_function("call_with_payment128", &context);
        let notify_function = self.generate_function("notify", &context);
        let notify_with_payment128_function =
            self.generate_function("notify_with_payment128", &context);

        quote! {
            #call_function
            #call_with_payment_function
            #call_with_payment128_function
            #notify_function
            #notify_with_payment128_function
        }
    }
}

impl ActExternalCanisterMethod {
    fn generate_function(&self, function_type: &str, context: &ActEcmContext) -> TokenStream {
        let is_oneway = function_type.contains("notify");

        let async_or_not = if is_oneway {
            quote! {}
        } else {
            quote! {async}
        };

        let function_name = format_ident!(
            "_{}_{}_{}_{}",
            context.cdk_name,
            function_type,
            context.canister_name,
            &self.name
        );

        let param_types = self.param_types_as_tuple(context.keyword_list);

        let cycles_param = if function_type.contains("with_payment128") {
            quote! { , cycles: u128 }
        } else if function_type.contains("with_payment") {
            quote! { , cycles: u64 }
        } else {
            quote! {}
        };

        let function_return_type = self.return_type.to_token_stream(context.keyword_list);
        let return_type = if is_oneway {
            quote! {Result<(), ic_cdk::api::call::RejectionCode>}
        } else {
            quote! {CallResult<(#function_return_type,)>}
        };

        let function_type_ident = format_ident!("{}", function_type);
        let api_call = quote! { ic_cdk::api::call::#function_type_ident };

        let method_name = &self.name;

        let cycles_arg = if function_type.contains("with_payment") {
            quote! { , cycles }
        } else {
            quote! {}
        };

        let await_or_not = if is_oneway {
            quote! {}
        } else {
            quote! {.await}
        };

        quote! {
            #[allow(non_snake_case)]
            #async_or_not fn #function_name(
                canister_id_principal: ic_cdk::export::Principal,
                params: #param_types
                #cycles_param
            ) -> #return_type {
                #api_call(
                    canister_id_principal,
                    #method_name,
                    params
                    #cycles_arg
                )#await_or_not
            }
        }
    }

    fn param_types_as_tuple(&self, keywords: &Vec<String>) -> TokenStream {
        let param_types: Vec<TokenStream> = self
            .params
            .iter()
            .map(|param| param.data_type.to_token_stream(keywords))
            .collect();

        let comma = if param_types.len() == 1 {
            quote! { , }
        } else {
            quote! {}
        };
        return quote! { (#(#param_types),*#comma) };
    }
}
