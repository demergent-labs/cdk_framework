use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;

use crate::{
    act::{
        node::{canister_method::QueryOrUpdateDefinition, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, IsCallable},
};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct QueryMethod {
    pub definition: QueryOrUpdateDefinition,
}

impl QueryMethod {
    fn generate_kybra_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];
        if self.is_async {
            args.push(quote! {composite = true});
        }
        if self.is_manual {
            args.push(quote! {manual_reply = true});
        }
        if let Some(guard_function) = &self.guard_function_name {
            args.push(quote! {guard = #guard_function});
        }

        quote!(#(#args),*)
    }

    fn generate_not_kybra_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];
        if self.is_async {
            args.push(quote! {composite = true});
            args.push(quote! {manual_reply = true});
        }
        if self.is_manual && !self.is_async {
            args.push(quote! {manual_reply = true});
        }
        if let Some(guard_function) = &self.guard_function_name {
            args.push(quote! {guard = #guard_function});
        }

        quote!(#(#args),*)
    }
}

impl Deref for QueryMethod {
    type Target = QueryOrUpdateDefinition;

    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}

impl Declare<Vec<String>> for QueryMethod {
    fn to_declaration(&self, keyword_list: &Vec<String>, _: String) -> Option<Declaration> {
        let function_declaration = self.generate_function_body(keyword_list);
        let macro_args = if self.cdk_name == "kybra" {
            self.generate_kybra_macro_args()
        } else {
            self.generate_not_kybra_macro_args()
        };
        Some(quote! {
            #[ic_cdk_macros::query(#macro_args)]
            #[candid::candid_method(query)]
            #function_declaration
        })
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.name.clone(), keyword_list)
    }
}

impl IsCallable for QueryMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        Some(self.return_type.clone())
    }
}
