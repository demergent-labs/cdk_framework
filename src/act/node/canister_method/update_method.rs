use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;

use crate::{
    act::{
        node::{canister_method::QueryOrUpdateDefinition, Context, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, IsCallable},
};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct UpdateMethod {
    pub definition: QueryOrUpdateDefinition,
}

impl UpdateMethod {
    fn generate_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];

        if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
            args.push(quote! {manual_reply = true});
        };
        if let Some(guard_function) = &self.guard_function_name {
            args.push(quote! {guard = #guard_function});
        };

        quote!(#(#args),*)
    }
}

impl Deref for UpdateMethod {
    type Target = QueryOrUpdateDefinition;

    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}

impl Declare<Context> for UpdateMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_declaration = self.generate_function_body(context);

        let macro_args = self.generate_macro_args();

        Some(quote! {
            #[ic_cdk_macros::update(#macro_args)]
            #[candid::candid_method(update)]
            #function_declaration
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.flatten_inlines(self.name.clone(), context)
    }
}

impl IsCallable for UpdateMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        Some(self.return_type.clone())
    }
}
