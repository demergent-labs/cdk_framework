use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    act::{
        node::{
            candid::TypeRef, canister_method, node_parts::mode::Mode, CandidType, Context, Param,
            ReturnType,
        },
        Declaration, Declare,
    },
    traits::{HasInlines, HasTypeRefs, IsCallable, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct Method {
    pub name: String,
    pub mode: Mode,
    pub params: Vec<Param>,
    pub return_type: ReturnType,
}

impl Method {
    pub fn new(name: String, mode: Mode, params: Vec<Param>, return_type: CandidType) -> Method {
        Method {
            name,
            mode,
            params,
            return_type: ReturnType::new(return_type),
        }
    }

    pub fn create_qualified_name(&self, canister_name: &String) -> String {
        format!(
            "{canister_name}_{method_name}",
            canister_name = canister_name,
            method_name = self.name
        )
    }

    fn generate_call_function(
        &self,
        canister_name: &String,
        function_type: &str,
        context: &Context,
        module_name: &Option<String>,
    ) -> TokenStream {
        let is_oneway = function_type.contains("notify");

        let async_or_not = if is_oneway {
            quote! {}
        } else {
            quote! {async}
        };

        let function_name = format_ident!("{}_{}_{}", function_type, canister_name, &self.name);

        let param_types = self.param_types_as_tuple(context, canister_name, module_name);

        let cycles_param = if function_type.contains("with_payment128") {
            quote! { , cycles: u128 }
        } else if function_type.contains("with_payment") {
            quote! { , cycles: u64 }
        } else {
            quote! {}
        };

        let function_return_type = self.return_type.to_type_annotation(
            context,
            self.create_qualified_name(canister_name),
            module_name,
        );
        let return_type = if is_oneway {
            quote! {Result<(), ic_cdk::api::call::RejectionCode>}
        } else {
            if function_return_type.to_string() == "" {
                quote! {ic_cdk::api::call::CallResult<()>}
            } else {
                quote! {ic_cdk::api::call::CallResult<(#function_return_type,)>}
            }
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
                canister_id_principal: candid::Principal,
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

    fn param_types_as_tuple(
        &self,
        context: &Context,
        canister_name: &String,
        module_name: &Option<String>,
    ) -> TokenStream {
        let param_types: Vec<_> = self
            .params
            .iter()
            .map(|param| {
                param.to_type_annotation(
                    context,
                    self.create_qualified_name(canister_name),
                    module_name,
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
    fn to_declaration(
        &self,
        context: &Context,
        canister_name: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        let call_function =
            self.generate_call_function(&canister_name, "call", &context, module_name);
        let call_with_payment_function =
            self.generate_call_function(&canister_name, "call_with_payment", &context, module_name);
        let call_with_payment128_function = self.generate_call_function(
            &canister_name,
            "call_with_payment128",
            &context,
            module_name,
        );
        let notify_function =
            self.generate_call_function(&canister_name, "notify", &context, module_name);
        let notify_with_payment128_function = self.generate_call_function(
            &canister_name,
            "notify_with_payment128",
            &context,
            module_name,
        );

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
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.flatten_inlines(
            self.create_qualified_name(&canister_name),
            context,
            module_name,
        )
    }
}

impl IsCallable for Method {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        Some(self.return_type.clone())
    }
}

impl HasTypeRefs for Method {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        canister_method::get_type_refs(&self.params, Some(&self.return_type))
    }
}
