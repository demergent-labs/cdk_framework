use self::canister_method::CanisterMethod;
use super::proclamation::Proclaim;
use super::Declaration;

pub mod canister_method;
pub mod data_type;
pub mod external_canister;
pub mod guard_function;
pub mod param;
pub mod traits;

pub use data_type::DataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;
pub use guard_function::GuardFunction;

#[derive(Clone)]
pub enum Node {
    CanisterMethod(CanisterMethod),
    DataType(DataType),
    ExternalCanister(ExternalCanister),
    GuardFunction(GuardFunction),
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
    ) -> Option<Declaration> {
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
            Node::GuardFunction(guard_function) => {
                guard_function.create_declaration(&context.keyword_list, parental_prefix)
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
            Node::GuardFunction(guard_function) => {
                guard_function.create_identifier(parental_prefix)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Vec<Declaration> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.collect_inline_declarations(context, parental_prefix)
            }
            Node::DataType(data_type) => {
                data_type.collect_inline_declarations(&context.keyword_list, parental_prefix)
            }
            Node::ExternalCanister(external_canister) => {
                external_canister.collect_inline_declarations(context, parental_prefix)
            }
            Node::GuardFunction(guard_function) => {
                guard_function.collect_inline_declarations(&context.keyword_list, parental_prefix)
            }
        }
    }
}
