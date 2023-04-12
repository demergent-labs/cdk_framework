use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{AsNode, Node},
        Declaration, Declare,
    },
    traits::{ToIdent, WithUserDefinedPrefix},
};

use super::Context;

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

impl Declare<Context> for GuardFunction {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        // TODO we will eventually need that _keyword list for when we analyze function names for keywords
        let name = self.name.with_user_defined_prefix().to_ident();
        let body = &self.body;

        Some(quote! {
            fn #name() -> Result<(), String> {
                #body
            }
        })
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![]
    }
}
