use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    act::{
        node::{param::Param, CandidType, Context},
        Declaration, Declare,
    },
    traits::{HasParams, HasReturnValue},
};

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: CandidType,
}

impl Method {
    fn create_qualified_name(&self, canister_name: &String) -> String {
        format!(
            "{canister_name}{method_name}",
            canister_name = canister_name,
            method_name = self.name
        )
    }

    fn generate_call_function(
        &self,
        canister_name: &String,
        function_type: &str,
        context: &Context,
    ) -> TokenStream {
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
            canister_name,
            &self.name
        );

        let param_types = self.param_types_as_tuple(&context.keyword_list, canister_name);

        let cycles_param = if function_type.contains("with_payment128") {
            quote! { , cycles: u128 }
        } else if function_type.contains("with_payment") {
            quote! { , cycles: u64 }
        } else {
            quote! {}
        };

        let function_return_type = self.create_return_type_annotation(
            &self.create_qualified_name(canister_name),
            &context.keyword_list,
        );
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

    fn param_types_as_tuple(&self, keywords: &Vec<String>, canister_name: &String) -> TokenStream {
        let param_types: Vec<_> = self
            .params
            .iter()
            .map(|param| {
                self.create_param_type_annotation(
                    param,
                    &self.create_qualified_name(canister_name),
                    keywords,
                )
            })
            .collect();

        let comma = if param_types.len() == 1 {
            quote! { , }
        } else {
            quote! {}
        };
        return quote! { (#(#param_types),*#comma) };
    }
}

impl Declare<Context> for Method {
    fn to_declaration(&self, context: &Context, canister_name: String) -> Option<Declaration> {
        let call_function = self.generate_call_function(&canister_name, "call", &context);
        let call_with_payment_function =
            self.generate_call_function(&canister_name, "call_with_payment", &context);
        let call_with_payment128_function =
            self.generate_call_function(&canister_name, "call_with_payment128", &context);
        let notify_function = self.generate_call_function(&canister_name, "notify", &context);
        let notify_with_payment128_function =
            self.generate_call_function(&canister_name, "notify_with_payment128", &context);

        Some(quote! {
            #call_function
            #call_with_payment_function
            #call_with_payment128_function
            #notify_function
            #notify_with_payment128_function
        })
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        canister_name: String,
    ) -> Vec<Declaration> {
        let param_declarations = self.collect_param_inline_declarations(
            &self.create_qualified_name(&canister_name),
            &context.keyword_list,
        );
        let return_declarations = self.collect_return_inline_declarations(
            &self.create_qualified_name(&canister_name),
            &context.keyword_list,
        );
        vec![param_declarations, return_declarations].concat()
    }
}

impl HasParams for Method {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}

impl HasReturnValue for Method {
    fn get_return_type(&self) -> CandidType {
        self.return_type.clone()
    }
}
