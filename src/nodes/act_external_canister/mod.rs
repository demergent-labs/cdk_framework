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

impl ActExternalCanister {}

impl ToTokenStream<&Vec<String>> for ActExternalCanister {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let cross_canister_call_functions: Vec<TokenStream> = self
            .methods
            .iter()
            .map(|method| {
                method.to_token_stream(ActEcmContext {
                    canister_name: self.name.clone(),
                    keyword_list: &keyword_list,
                })
            })
            .collect();
        quote! { #(#cross_canister_call_functions)*}
    }
}
