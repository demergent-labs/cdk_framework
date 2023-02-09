use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::act::{
    self,
    declaration::ToDeclaration,
    node::{
        canister_method::FnParam,
        traits::{HasParams, HasReturnValue},
        DataType,
    },
};

use super::PublicCanisterMethod;

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct UpdateMethod {
    pub body: TokenStream,
    pub params: Vec<FnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub function_guard_name: Option<String>,
}

impl UpdateMethod {
    fn generate_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];

        if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
            args.push(quote! {manual_reply = true});
        };
        if let Some(guard_function) = &self.function_guard_name {
            args.push(quote! {guard = #guard_function});
        };

        quote!(#(#args),*)
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

impl ToDeclaration<Vec<String>> for UpdateMethod {
    fn create_code(&self, keyword_list: &Vec<String>, _: String) -> Option<TokenStream> {
        let function_declaration = self.generate_function_declaration(keyword_list);

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

    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        let param_declarations = self.create_param_declarations(keyword_list);
        let result_declarations = self.create_return_type_declarations(keyword_list);
        act::combine_maps(param_declarations, result_declarations)
    }
}

impl HasParams for UpdateMethod {
    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }

    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("{}ParamNum{}", self.name, param_index)
    }
}

impl HasReturnValue for UpdateMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }

    fn create_return_type_prefix(&self) -> String {
        format!("{}ReturnType", self.name)
    }
}
