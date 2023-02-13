use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use self::external_canister_method::EcmContext;
use crate::act::proclamation::Proclaim;

pub mod external_canister_method;

pub use external_canister_method::ExternalCanisterMethod;

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

impl Proclaim<TokenStreamContext<'_>> for ExternalCanister {
    fn create_declaration(
        &self,
        context: &TokenStreamContext<'_>,
        _: String,
    ) -> Option<TokenStream> {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .filter_map(|method| {
                method.create_declaration(
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

    fn create_inline_declarations(
        &self,
        context: &TokenStreamContext<'_>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.methods.create_inline_declarations(
            &EcmContext {
                canister_name: self.name.clone(),
                keyword_list: &context.keyword_list,
                cdk_name: context.cdk_name,
            },
            parental_prefix,
        )
    }
}
