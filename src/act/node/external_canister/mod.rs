use quote::quote;

use self::external_canister_method::EcmContext;

pub mod external_canister_method;

pub use external_canister_method::ExternalCanisterMethod;

use super::{declaration::Declare, Declaration, NodeContext};

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<ExternalCanisterMethod>,
}

impl Declare<NodeContext> for ExternalCanister {
    fn to_declaration(&self, context: &NodeContext, _: String) -> Option<Declaration> {
        let cross_canister_call_functions: Vec<_> = self
            .methods
            .iter()
            .filter_map(|method| {
                method.to_declaration(
                    &EcmContext {
                        canister_name: self.name.clone(),
                        keyword_list: context.keyword_list.clone(),
                        cdk_name: context.cdk_name.clone(),
                    },
                    self.name.clone(),
                )
            })
            .collect();
        Some(quote! { #(#cross_canister_call_functions)*})
    }

    fn collect_inline_declarations(&self, context: &NodeContext, _: String) -> Vec<Declaration> {
        self.methods.iter().fold(vec![], |acc, method| {
            vec![
                acc,
                method.collect_inline_declarations(
                    &EcmContext {
                        canister_name: self.name.clone(),
                        keyword_list: context.keyword_list.clone(),
                        cdk_name: context.cdk_name.clone(),
                    },
                    self.name.clone(),
                ),
            ]
            .concat()
        })
    }
}
