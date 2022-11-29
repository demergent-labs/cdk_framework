use proc_macro2::TokenStream;
use quote::quote;

use crate::ToTokenStream;

#[derive(Clone)]
pub struct ActPreUpgradeMethod {
    pub body: TokenStream,
}

impl ToTokenStream<()> for ActPreUpgradeMethod {
    fn to_token_stream(&self, _context: ()) -> TokenStream {
        let body = &self.body;
        quote! {
            #[ic_cdk_macros::pre_upgrade]
            fn _azle_pre_upgrade() {
                #body
            }
        }
    }
}
