use crate::act::{
    node::{CandidType, CanisterMethod, Context, GuardFunction, Service},
    Declaration, Declare,
};

#[derive(Clone)]
pub enum Node {
    CanisterMethod(CanisterMethod),
    CandidType(CandidType),
    Service(Service),
    GuardFunction(GuardFunction),
}

pub trait AsNode {
    fn as_node(self) -> Node;
}

impl Declare<Context> for Node {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.to_declaration(context, inline_name)
            }
            Node::CandidType(candid_type) => {
                candid_type.to_declaration(&context.keyword_list, inline_name)
            }
            Node::Service(service) => service.to_declaration(context, inline_name),
            Node::GuardFunction(guard_function) => {
                guard_function.to_declaration(&context.keyword_list, inline_name)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        match self {
            Node::CanisterMethod(canister_method) => {
                canister_method.collect_inline_declarations(context, inline_name)
            }
            Node::CandidType(candid_type) => {
                candid_type.collect_inline_declarations(&context.keyword_list, inline_name)
            }
            Node::Service(service) => service.collect_inline_declarations(context, inline_name),
            Node::GuardFunction(guard_function) => {
                guard_function.collect_inline_declarations(&context.keyword_list, inline_name)
            }
        }
    }
}
