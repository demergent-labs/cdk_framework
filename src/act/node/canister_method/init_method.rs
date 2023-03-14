use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{Context, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, IsCallable, ToIdent},
};

#[derive(Clone)]
pub struct InitMethod {
    pub guard_function_name: Option<String>,
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl InitMethod {
    fn get_name(&self, context: &Context) -> String {
        format!("_{}_init", context.cdk_name.to_lowercase())
    }
}

impl Declare<Context> for InitMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = self.get_name(context).to_ident();
        let body = &self.body;
        let params = self
            .create_parameter_list_token_stream(&self.get_name(&context), &context.keyword_list);
        let macro_args = match &self.guard_function_name {
            Some(guard_function_name) => quote! {guard = #guard_function_name},
            None => quote!(),
        };

        Some(quote! {
            #[ic_cdk_macros::init(#macro_args)]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(&context), &context.keyword_list)
    }
}

impl IsCallable for InitMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        None
    }
}
