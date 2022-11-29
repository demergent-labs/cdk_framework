use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{nodes::ActFnParam, ActDataType, ToTokenStream, ToTokenStreams};

#[derive(Clone, Debug)]
pub struct ActExternalCanisterMethod {
    pub name: String,
    pub params: Vec<ActFnParam>,
    pub return_type: ActDataType,
}

impl ActExternalCanisterMethod {
    pub fn to_token_stream(&self, canister_name: &String, context: &Vec<String>) -> TokenStream {
        let call_function = self.generate_call_function(canister_name, context);
        let call_with_payment_function =
            self.generate_call_with_payment_function(canister_name, context);
        let call_with_payment128_function =
            self.generate_call_with_payment128_function(canister_name, context);
        let notify_function = self.generate_notify_function(canister_name, context);
        let notify_with_payment128_function =
            self.generate_notify_with_payment128_function(canister_name, context);

        quote! {
            #call_function
            #call_with_payment_function
            #call_with_payment128_function
            #notify_function
            #notify_with_payment128_function
        }
    }

    pub fn params_as_args_list(&self) -> TokenStream {
        let param_names = self.param_names();

        let comma = if param_names.len() == 0 {
            quote! {}
        } else {
            quote! {,}
        };
        return quote! { #(#param_names),*#comma };
    }

    fn generate_call_function(&self, canister_name: &String, context: &Vec<String>) -> TokenStream {
        let function_name = format_ident!("_azle_call_{}_{}", canister_name, &self.name);

        let params = vec![
            vec![quote! { canister_id_principal: ic_cdk::export::Principal }],
            self.params.to_token_streams(context),
        ]
        .concat();

        let function_return_type = self.return_type.to_token_stream(context);
        let method_name = &self.name;
        let args = self.params_as_tuple();

        quote! {
            #[allow(non_snake_case)]
            async fn #function_name(#(#params),*) -> CallResult<(#function_return_type,)> {
                ic_cdk::api::call::call(
                    canister_id_principal,
                    #method_name,
                    #args
                ).await
            }
        }
    }

    fn generate_call_with_payment_function(
        &self,
        canister_name: &String,
        context: &Vec<String>,
    ) -> TokenStream {
        let function_name =
            format_ident!("_azle_call_with_payment_{}_{}", canister_name, &self.name);

        let params = vec![
            vec![quote! { canister_id_principal: ic_cdk::export::Principal }],
            self.params.to_token_streams(context),
            vec![quote! { cycles: u64 }],
        ]
        .concat();

        let function_return_type = self.return_type.to_token_stream(context);
        let method_name = &self.name;
        let args = self.params_as_tuple();

        quote! {
            #[allow(non_snake_case)]
            async fn #function_name(#(#params),*) -> CallResult<(#function_return_type,)> {
                ic_cdk::api::call::call_with_payment(
                    canister_id_principal,
                    #method_name,
                    #args,
                    cycles
                ).await
            }
        }
    }

    fn generate_call_with_payment128_function(
        &self,
        canister_name: &String,
        context: &Vec<String>,
    ) -> TokenStream {
        let function_name = format_ident!(
            "_azle_call_with_payment128_{}_{}",
            canister_name,
            &self.name
        );

        let params = vec![
            vec![quote! { canister_id_principal: ic_cdk::export::Principal }],
            self.params.to_token_streams(context),
            vec![quote! { cycles: u128 }],
        ]
        .concat();

        let function_return_type = self.return_type.to_token_stream(context);
        let method_name = &self.name;
        let args = self.params_as_tuple();

        quote! {
            #[allow(non_snake_case)]
            async fn #function_name(#(#params),*) -> CallResult<(#function_return_type,)> {
                ic_cdk::api::call::call_with_payment128(
                    canister_id_principal,
                    #method_name,
                    #args,
                    cycles
                ).await
            }
        }
    }

    fn generate_notify_function(
        &self,
        canister_name: &String,
        context: &Vec<String>,
    ) -> TokenStream {
        let function_name = format_ident!("_azle_notify_{}_{}", canister_name, &self.name);

        let params = vec![
            vec![quote! { canister_id_principal: ic_cdk::export::Principal }],
            self.params.to_token_streams(context),
        ]
        .concat();

        let method_name = &self.name;
        let args = self.params_as_tuple();

        quote! {
            #[allow(non_snake_case)]
            fn #function_name(#(#params),*) -> Result<(), ic_cdk::api::call::RejectionCode> {
                ic_cdk::api::call::notify(
                    canister_id_principal,
                    #method_name,
                    #args
                )
            }
        }
    }

    fn generate_notify_with_payment128_function(
        &self,
        canister_name: &String,
        context: &Vec<String>,
    ) -> TokenStream {
        let function_name = format_ident!(
            "_azle_notify_with_payment128_{}_{}",
            canister_name,
            &self.name
        );

        let params = vec![
            vec![quote! { canister_id_principal: ic_cdk::export::Principal }],
            self.params.to_token_streams(context),
            vec![quote! { cycles: u128 }],
        ]
        .concat();

        let method_name = &self.name;
        let args = self.params_as_tuple();

        quote! {
            #[allow(non_snake_case)]
            fn #function_name(#(#params),*) -> Result<(), ic_cdk::api::call::RejectionCode> {
                ic_cdk::api::call::notify_with_payment128(
                    canister_id_principal,
                    #method_name,
                    #args,
                    cycles
                )
            }
        }
    }

    fn params_as_tuple(&self) -> TokenStream {
        let param_names = self.param_names();

        let comma = if param_names.len() == 1 {
            quote! { , }
        } else {
            quote! {}
        };
        return quote! { (#(#param_names),*#comma) };
    }

    fn param_names(&self) -> Vec<TokenStream> {
        self.params
            .iter()
            .map(|param| {
                let param_ident = format_ident!("{}", param.name);
                quote! { #param_ident }
            })
            .collect()
    }
}
