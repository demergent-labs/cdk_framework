pub use self::declaration::Declaration;
use self::declaration::Declare;

pub mod candid;
pub mod canister_method;
pub mod declaration;
pub mod external_canister;
pub mod guard_function;
pub mod param;
pub mod traits;

pub use candid::CandidType;
pub use canister_method::CanisterMethod;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;
pub use guard_function::GuardFunction;
pub use param::Param;

#[derive(Clone)]
pub enum Node {
    CanisterMethod(CanisterMethod),
    CandidType(CandidType),
    ExternalCanister(ExternalCanister),
    GuardFunction(GuardFunction),
}

pub trait AsNode {
    fn as_node(self) -> Node;
}

#[derive(Clone)]
pub struct NodeContext {
    pub keyword_list: Vec<String>,
    pub cdk_name: String,
}

impl Declare<NodeContext> for Node {
    fn to_declaration(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Option<Declaration> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.to_declaration(context, parental_prefix)
            }
            Node::CandidType(candid_type) => {
                candid_type.to_declaration(&context.keyword_list, parental_prefix)
            }
            Node::ExternalCanister(external_canister) => {
                external_canister.to_declaration(context, parental_prefix)
            }
            Node::GuardFunction(guard_function) => {
                guard_function.to_declaration(&context.keyword_list, parental_prefix)
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
            Node::CandidType(candid_type) => {
                candid_type.collect_inline_declarations(&context.keyword_list, parental_prefix)
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
