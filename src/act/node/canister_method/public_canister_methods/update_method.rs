use proc_macro2::TokenStream;
use quote::quote;

use crate::act::node::{
    param::Param,
    proclamation::Proclaim,
    traits::{HasParams, HasReturnValue},
    DataType, Declaration,
};

use super::PublicCanisterMethod;

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct UpdateMethod {
    pub body: TokenStream,
    pub params: Vec<Param>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub guard_function_name: Option<String>,
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

impl Proclaim<Vec<String>> for UpdateMethod {
    fn create_declaration(&self, keyword_list: &Vec<String>, _: String) -> Option<Declaration> {
        let function_declaration = self.generate_function_body(keyword_list);

        let macro_args = self.generate_macro_args();

        Some(quote! {
            #[ic_cdk_macros::update(#macro_args)]
            #[candid::candid_method(update)]
            #function_declaration
        })
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> Vec<Declaration> {
        let param_declarations = self.collect_param_inline_declarations(keyword_list, &self.name);
        let return_declarations = self.collect_return_inline_declarations(keyword_list, &self.name);
        vec![param_declarations, return_declarations].concat()
    }
}

impl HasParams for UpdateMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}

impl HasReturnValue for UpdateMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }
}

impl PublicCanisterMethod for UpdateMethod {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_body(&self) -> TokenStream {
        self.body.clone()
    }

    fn get_cdk_name(&self) -> String {
        self.cdk_name.clone()
    }

    fn is_manual(&self) -> bool {
        self.is_manual
    }

    fn is_async(&self) -> bool {
        self.is_async
    }
}
