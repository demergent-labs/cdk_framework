use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;

use crate::{
    act::{
        node::{canister_method::QueryOrUpdateDefinition, Context, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, IsCallable, WithUserDefinedPrefix},
};

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct UpdateMethod {
    pub definition: QueryOrUpdateDefinition,
}

impl UpdateMethod {
    fn generate_macro_args(&self, cdk_name: &str) -> TokenStream {
        let user_defined_name = &self.name;

        let mut args: Vec<TokenStream> = vec![quote! {name = #user_defined_name}];

        if self.is_manual || (self.is_async && cdk_name != "kybra") {
            args.push(quote! {manual_reply = true});
        };
        if let Some(guard_function) = &self.guard_function_name {
            let prefixed_guard_function_name = guard_function.with_user_defined_prefix();
            args.push(quote! {guard = #prefixed_guard_function_name});
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
    fn to_declaration(
        &self,
        context: &Context,
        _: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        // let user_defined_name = &self.name;
        let function_declaration = self.generate_function_body(context, module_name);
        let macro_args = self.generate_macro_args(&context.cdk_name);

        Some(quote! {
            #[ic_cdk_macros::update(#macro_args)]
            #[candid::candid_method(update)]
            #function_declaration
        })
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        _: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.name.clone(), context, module_name)
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
