use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{candid::TypeRef, CandidType, Context, Param, ReturnType},
    traits::{HasTypeRefs, IsCallable, ToIdent, ToTypeAnnotation, WithUserDefinedPrefix},
};

use super::canister_method;

#[derive(Clone, Debug)]
pub struct QueryOrUpdateDefinition {
    pub is_async: bool,
    pub is_manual: bool,
    pub guard_function_name: Option<String>,
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: ReturnType,
    pub body: TokenStream,
}

impl QueryOrUpdateDefinition {
    pub fn new(
        is_async: bool,
        is_manual: bool,
        guard_function_name: Option<String>,
        name: String,
        params: Vec<Param>,
        return_type: CandidType,
        body: TokenStream,
    ) -> QueryOrUpdateDefinition {
        QueryOrUpdateDefinition {
            is_async,
            is_manual,
            guard_function_name,
            name,
            params,
            return_type: ReturnType::new(return_type),
            body,
        }
    }

    pub fn generate_function_body(&self, context: &Context) -> TokenStream {
        let function_name = self.name.with_user_defined_prefix().to_ident();
        let params = self.create_parameter_list_token_stream(&self.name, context);

        let function_body = &self.body;

        let return_type_token = self
            .return_type
            .to_type_annotation(context, self.name.clone());

        let wrapped_return_type =
            if self.is_manual || (self.is_async && context.cdk_name != "kybra") {
                quote! {
                    ic_cdk::api::call::ManualReply<(#return_type_token)>
                }
            } else {
                return_type_token
            };

        quote! {
            async fn #function_name(#params) -> (#wrapped_return_type) {
                #function_body
            }
        }
    }
}

impl IsCallable for QueryOrUpdateDefinition {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        Some(self.return_type.clone())
    }
}

impl HasTypeRefs for QueryOrUpdateDefinition {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        canister_method::get_type_refs(&self.params, Some(&self.return_type))
    }
}
