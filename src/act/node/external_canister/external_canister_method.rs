use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::act::{
    declaration::ToDeclaration,
    node::{
        canister_method::FnParam,
        traits::{has_params::HasParams, has_return_value::HasReturnValue},
        DataType,
    },
};

#[derive(Clone, Debug)]
pub struct ExternalCanisterMethod {
    pub name: String,
    pub params: Vec<FnParam>,
    pub return_type: DataType,
}

#[derive(Clone)]
pub struct EcmContext<'a> {
    pub canister_name: String,
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToDeclaration<EcmContext<'_>> for ExternalCanisterMethod {
    fn create_code(&self, context: &EcmContext<'_>, _: String) -> Option<TokenStream> {
        let call_function = self.generate_function("call", &context);
        let call_with_payment_function = self.generate_function("call_with_payment", &context);
        let call_with_payment128_function =
            self.generate_function("call_with_payment128", &context);
        let notify_function = self.generate_function("notify", &context);
        let notify_with_payment128_function =
            self.generate_function("notify_with_payment128", &context);

        Some(quote! {
            #call_function
            #call_with_payment_function
            #call_with_payment128_function
            #notify_function
            #notify_with_payment128_function
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_child_declarations(
        &self,
        context: &EcmContext<'_>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        let mut declarations = self.create_param_declarations(context.keyword_list);
        declarations.extend(self.create_return_type_declarations(context.keyword_list));
        declarations
    }
}

impl HasReturnValue for ExternalCanisterMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }

    fn create_return_type_prefix(&self) -> String {
        format!("ExternalCanisterMethod{}", self.name)
    }
}

impl HasParams for ExternalCanisterMethod {
    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("ExternalCanisterMethod{}ParamNum{}", self.name, param_index)
    }
}

impl ExternalCanisterMethod {
    fn generate_function(&self, function_type: &str, context: &EcmContext) -> TokenStream {
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

        let function_return_type = self.create_return_type_annotation(context.keyword_list);
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
            .enumerate()
            .filter_map(|(index, _)| self.create_param_type_annotation(index, keywords))
            .collect();

        let comma = if param_types.len() == 1 {
            quote! { , }
        } else {
            quote! {}
        };
        return quote! { (#(#param_types),*#comma) };
    }
}
