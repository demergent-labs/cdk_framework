use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{candid::TypeRef, Context, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, HasTypeRefs, IsCallable, ToIdent},
};

use super::canister_method;

#[derive(Clone)]
pub struct InitMethod {
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl InitMethod {
    fn get_name(&self) -> String {
        "init".to_string()
    }
}

impl Declare<Context> for InitMethod {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let function_name = self.get_name().to_ident();
        let body = &self.body;
        let params = self.create_parameter_list_token_stream(&self.get_name(), context);

        Some(quote! {
            #[ic_cdk_macros::init]
            #[candid::candid_method(init)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(), context)
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

impl HasTypeRefs for InitMethod {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        canister_method::get_type_refs(&self.params, None)
    }
}
