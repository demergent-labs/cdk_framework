use quote::quote;

use self::external_canister_method::EcmContext;

pub mod external_canister_method;

pub use external_canister_method::ExternalCanisterMethod;

use super::{proclamation::Proclaim, Declaration, NodeContext};

#[derive(Clone, Debug)]
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<ExternalCanisterMethod>,
}

impl Proclaim<NodeContext> for ExternalCanister {
    fn create_declaration(&self, context: &NodeContext, _: String) -> Option<Declaration> {
        let cross_canister_call_functions: Vec<_> = self
            .methods
            .iter()
            .filter_map(|method| {
                method.create_declaration(
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

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
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
