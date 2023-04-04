use proc_macro2::TokenStream;

pub trait ToTokenStream<C> {
    fn to_token_stream(&self, context: &C, inline_name: &str) -> TokenStream;
}
