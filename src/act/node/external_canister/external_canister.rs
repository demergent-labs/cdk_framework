use quote::quote;

use super::Method;
use crate::act::node::{declaration::Declare, AsNode, Context, Declaration, Node};

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<Method>,
}

impl AsNode for ExternalCanister {
    fn as_node(self) -> Node {
        Node::ExternalCanister(self)
    }
}

impl Declare<Context> for ExternalCanister {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let cross_canister_call_functions: Vec<_> = self
            .methods
            .iter()
            .filter_map(|method| method.to_declaration(context, self.name.clone()))
            .collect();
        Some(quote! { #(#cross_canister_call_functions)*})
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.methods.iter().fold(vec![], |acc, method| {
            vec![
                acc,
                method.collect_inline_declarations(context, self.name.clone()),
            ]
            .concat()
        })
    }
}
