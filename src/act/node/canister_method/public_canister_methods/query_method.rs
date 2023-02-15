use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::act::{
    self,
    node::{
        canister_method::FnParam,
        traits::{HasParams, HasReturnValue},
        DataType,
    },
    proclamation::Proclaim,
};

use super::PublicCanisterMethod;

/// Describes a Rust canister method function body
#[derive(Debug, Clone)]
pub struct QueryMethod {
    pub body: TokenStream,
    pub params: Vec<FnParam>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub function_guard_name: Option<String>,
}

impl QueryMethod {
    fn generate_kybra_macro_args(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];
        if self.is_async {
            args.push(quote! {composite = true});
        };
        if self.is_manual {
            args.push(quote! {manual_reply = true});
        };
        if let Some(guard_function) = &self.function_guard_name {
            args.push(quote! {guard = #guard_function});
        };

        quote!(#(#args),*)
    }

    fn generate_not_kybra_macro_args(&self) -> TokenStream {
        if self.is_async {
            quote! {composite = true, manual_reply = true}
        } else if self.is_manual {
            quote! {manual_reply = true}
        } else {
            quote! {}
        }
    }
}

impl HasParams for QueryMethod {
    fn create_param_prefix(&self, param_index: usize) -> String {
        format!("{}ParamNum{}", self.name, param_index)
    }

    fn get_params(&self) -> Vec<FnParam> {
        self.params.clone()
    }
}

impl HasReturnValue for QueryMethod {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }

    fn create_return_type_prefix(&self) -> String {
        format!("{}ReturnType", self.name)
    }
}

impl PublicCanisterMethod for QueryMethod {
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

impl Proclaim<Vec<String>> for QueryMethod {
    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        _: String,
    ) -> HashMap<String, TokenStream> {
        let param_declarations = self.collect_param_inline_types(keyword_list);
        let return_declarations = self.create_return_type_declarations(keyword_list);
        act::combine_maps(param_declarations, return_declarations)
    }

    fn create_declaration(&self, keyword_list: &Vec<String>, _: String) -> Option<TokenStream> {
        let function_declaration = self.generate_function_declaration(keyword_list);
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

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }
}
