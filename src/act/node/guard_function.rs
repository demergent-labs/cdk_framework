use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{AsNode, Node},
        Declaration, Declare,
    },
    traits::ToIdent,
};

#[derive(Debug, Clone)]
pub struct GuardFunction {
    pub body: TokenStream,
    pub name: String,
}

impl AsNode for GuardFunction {
    fn as_node(self) -> Node {
        Node::GuardFunction(self)
    }
}

impl Declare<Vec<String>> for GuardFunction {
    fn to_declaration(&self, _keyword_list: &Vec<String>, _: String) -> Option<Declaration> {
        // TODO we will eventually need that _keyword list for when we analyze function names for keywords
        let name = self.name.to_ident();
        let body = &self.body;

        Some(quote! {
            fn #name() -> Result<(), String> {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<Declaration> {
        vec![]
    }
}
