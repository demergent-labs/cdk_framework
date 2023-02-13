use std::collections::HashMap;

use self::canister_method::CanisterMethod;

pub mod canister_method;
pub mod data_type;
pub mod external_canister;
pub mod function_guard;
pub mod traits;

pub use data_type::DataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;
pub use function_guard::FunctionGuard;
use proc_macro2::TokenStream;

use super::proclamation::Proclaim;

#[derive(Clone)]
pub enum Node {
    CanisterMethod(CanisterMethod),
    DataType(DataType),
    ExternalCanister(ExternalCanister),
    StableBTreeMap,
    FunctionGuard(FunctionGuard),
}

#[derive(Clone)]
pub struct NodeContext {
    pub keyword_list: Vec<String>,
    pub cdk_name: String,
}

impl Proclaim<NodeContext> for Node {
    fn create_declaration(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.create_declaration(context, parental_prefix)
            }
            Node::DataType(data_type) => {
                data_type.create_declaration(&context.keyword_list, parental_prefix)
            }
            Node::ExternalCanister(external_canister) => {
                external_canister.create_declaration(context, parental_prefix)
            }
            Node::StableBTreeMap => todo!(),
            Node::FunctionGuard(function_guard) => {
                function_guard.create_declaration(&context.keyword_list, parental_prefix)
            }
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.create_identifier(parental_prefix)
            }
            Node::DataType(data_type) => data_type.create_identifier(parental_prefix),
            Node::ExternalCanister(external_canister) => {
                external_canister.create_identifier(parental_prefix)
            }
            Node::StableBTreeMap => todo!(),
            Node::FunctionGuard(function_guard) => {
                function_guard.create_identifier(parental_prefix)
            }
        }
    }

    fn create_inline_declarations(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.create_inline_declarations(context, parental_prefix)
            }
            Node::DataType(data_type) => {
                data_type.create_inline_declarations(&context.keyword_list, parental_prefix)
            }
            Node::ExternalCanister(external_canister) => {
                external_canister.create_inline_declarations(context, parental_prefix)
            }
            Node::StableBTreeMap => todo!(),
            Node::FunctionGuard(function_guard) => {
                function_guard.create_inline_declarations(&context.keyword_list, parental_prefix)
            }
        }
    }
}
