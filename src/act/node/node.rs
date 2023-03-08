use crate::act::{
    node::{CandidType, CanisterMethod, Context, ExternalCanister, GuardFunction},
    Declaration, Declare,
};

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

impl Declare<Context> for Node {
    fn to_declaration(&self, context: &Context, parental_prefix: String) -> Option<Declaration> {
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
        context: &Context,
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
