use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{candid::TypeRef, Context, Param, ReturnType},
        Declaration, Declare,
    },
    traits::{HasInlines, HasTypeRefs, IsCallable, ToIdent, WithUserDefinedPrefix},
};

use super::canister_method;

#[derive(Clone)]
pub struct PostUpgradeMethod {
    pub guard_function_name: Option<String>,
    pub params: Vec<Param>,
    pub body: TokenStream,
}

impl PostUpgradeMethod {
    fn get_name(&self) -> String {
        "post_upgrade".to_string()
    }
}

impl Declare<Context> for PostUpgradeMethod {
    fn to_declaration(
        &self,
        context: &Context,
        _: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        let function_name = self.get_name().to_ident();
        let body = &self.body;
        let params =
            self.create_parameter_list_token_stream(&self.get_name(), context, module_name);
        let macro_args = match &self.guard_function_name {
            Some(guard_function_name) => {
                let prefixed_guard_function_name = guard_function_name.with_user_defined_prefix();
                quote! {guard = #prefixed_guard_function_name}
            }
            None => quote!(),
        };

        Some(quote! {
            #[ic_cdk_macros::post_upgrade(#macro_args)]
            fn #function_name(#params) {
                #body
            }
        })
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        _: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(), context, module_name)
    }
}

impl IsCallable for PostUpgradeMethod {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        None
    }
}

impl HasTypeRefs for PostUpgradeMethod {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        canister_method::get_type_refs(&self.params, None)
    }
}
