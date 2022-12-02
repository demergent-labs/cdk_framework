use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::ToTokenStream;

#[derive(Clone)]
pub struct ActPreUpgradeMethod {
    pub body: TokenStream,
}

impl ToTokenStream<&String> for ActPreUpgradeMethod {
    fn to_token_stream(&self, cdk_name: &String) -> TokenStream {
        let function_name = format_ident!("_{}_pre_upgrade", cdk_name.to_lowercase());
        let body = &self.body;
        quote! {
            #[ic_cdk_macros::pre_upgrade]
            fn #function_name() {
                #body
            }
        }
    }
}
