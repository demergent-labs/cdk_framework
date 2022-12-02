use proc_macro2::TokenStream;
use quote::quote;

use crate::ToTokenStream;

pub use act_external_canister_method::ActExternalCanisterMethod;

use self::act_external_canister_method::ActEcmContext;

pub mod act_external_canister_method;

#[derive(Clone, Debug)]
pub struct ActExternalCanister {
    pub name: String,
    pub methods: Vec<ActExternalCanisterMethod>,
}

#[derive(Clone)]
pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToTokenStream<TokenStreamContext<'_>> for ActExternalCanister {
    fn to_token_stream(&self, context: TokenStreamContext) -> TokenStream {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .map(|method| {
                method.to_token_stream(ActEcmContext {
                    canister_name: self.name.clone(),
                    keyword_list: &context.keyword_list,
                    cdk_name: context.cdk_name,
                })
            })
            .collect();
        quote! { #(#cross_canister_call_functions)*}
    }
}
