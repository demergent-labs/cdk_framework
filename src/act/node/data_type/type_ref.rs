use crate::{traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::ToTokens;

// TODO what's more(see below) I think we don't even need it for the old version anymore
// TODO I think this is just temporary for that the old versions of kybra and azle will still compile

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTokenStream<&Vec<String>> for TypeRef {
    fn to_token_stream(&self, _: &Vec<String>) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}
