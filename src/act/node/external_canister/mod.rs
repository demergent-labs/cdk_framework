use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use self::external_canister_method::EcmContext;
use crate::act::proclamation::Proclaim;

pub mod external_canister_method;

pub use external_canister_method::ExternalCanisterMethod;

use super::NodeContext;

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<ExternalCanisterMethod>,
}

impl Proclaim<NodeContext> for ExternalCanister {
    fn create_declaration(&self, context: &NodeContext, _: String) -> Option<TokenStream> {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .filter_map(|method| {
                method.create_declaration(
                    &EcmContext {
                        canister_name: self.name.clone(),
                        keyword_list: context.keyword_list.clone(),
                        cdk_name: context.cdk_name.clone(),
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

    fn collect_inline_declarations(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.methods.collect_inline_declarations(
            &EcmContext {
                canister_name: self.name.clone(),
                keyword_list: context.keyword_list.clone(),
                cdk_name: context.cdk_name.clone(),
            },
            parental_prefix,
        )
    }
}
