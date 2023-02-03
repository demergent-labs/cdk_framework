use proc_macro2::TokenStream;
use quote::quote;

use crate::ToTokenStream;

pub use external_canister_method::ExternalCanisterMethod;

use self::external_canister_method::EcmContext;

pub mod external_canister_method;

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<ExternalCanisterMethod>,
}

#[derive(Clone)]
pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToTokenStream<TokenStreamContext<'_>> for ExternalCanister {
    fn to_token_stream(&self, context: &TokenStreamContext) -> TokenStream {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .map(|method| {
                method.to_token_stream(&EcmContext {
                    canister_name: self.name.clone(),
                    keyword_list: &context.keyword_list,
                    cdk_name: context.cdk_name,
                })
            })
            .collect();
        quote! { #(#cross_canister_call_functions)*}
    }
}
