use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{nodes::ActFnParam, ToTokenStream, ToTokenStreams};

#[derive(Clone)]
pub struct ActPostUpgradeMethod {
    pub params: Vec<ActFnParam>,
    pub body: TokenStream,
}

pub struct TokenStreamContext<'a> {
    pub keyword_list: &'a Vec<String>,
    pub cdk_name: &'a String,
}

impl ToTokenStream<TokenStreamContext<'_>> for ActPostUpgradeMethod {
    fn to_token_stream(&self, context: TokenStreamContext) -> TokenStream {
        let function_name = format_ident!("_{}_post_upgrade", context.cdk_name.to_lowercase());
        let body = &self.body;
        let params = &self.params.to_token_streams(context.keyword_list);
        quote! {
            #[ic_cdk_macros::post_upgrade]
            fn #function_name(#(#params),*) {
                #body
            }
        }
    }
}
