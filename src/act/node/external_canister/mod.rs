use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

pub use external_canister_method::ExternalCanisterMethod;

use crate::act::declaration::ToDeclaration;

use self::external_canister_method::EcmContext;

pub mod external_canister_method;

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<ExternalCanisterMethod>,
}

#[derive(Clone)]
pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToDeclaration<TokenStreamContext<'_>> for ExternalCanister {
    fn create_code(&self, context: &TokenStreamContext<'_>, _: String) -> Option<TokenStream> {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .filter_map(|method| {
                method.create_code(
                    &EcmContext {
                        canister_name: self.name.clone(),
                        keyword_list: &context.keyword_list,
                        cdk_name: context.cdk_name,
                    },
                    self.name.clone(),
                )
            })
            .collect();
        Some(quote! { #(#cross_canister_call_functions)*})
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_child_declarations(
        &self,
        context: &TokenStreamContext<'_>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.methods.create_child_declarations(
            &EcmContext {
                canister_name: self.name.clone(),
                keyword_list: &context.keyword_list,
                cdk_name: context.cdk_name,
            },
            parental_prefix,
        )
    }
}
