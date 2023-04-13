use super::{
    HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
    QueryMethod, UpdateMethod,
};
use crate::act::{
    node::{candid::TypeRef, AsNode, Context, Node, Param, ReturnType},
    Declaration, Declare,
};

#[derive(Clone)]
pub enum CanisterMethod {
    Update(UpdateMethod),
    Query(QueryMethod),
    Init(InitMethod),
    PreUpgrade(PreUpgradeMethod),
    PostUpgrade(PostUpgradeMethod),
    InspectMessage(InspectMessageMethod),
    Heartbeat(HeartbeatMethod),
}

impl AsNode for CanisterMethod {
    fn as_node(self) -> Node {
        Node::CanisterMethod(self)
    }
}

impl Declare<Context> for CanisterMethod {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        let f = |m: &dyn Declare<Context>| m.to_declaration(context, inline_name);
        match self {
            CanisterMethod::Update(update_method) => f(update_method),
            CanisterMethod::Query(query_method) => f(query_method),
            CanisterMethod::Init(init) => f(init),
            CanisterMethod::PreUpgrade(pre_upgrade) => f(pre_upgrade),
            CanisterMethod::PostUpgrade(post_upgrade) => f(post_upgrade),
            CanisterMethod::InspectMessage(inspect_message) => f(inspect_message),
            CanisterMethod::Heartbeat(heartbeat) => f(heartbeat),
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        let f = |m: &dyn Declare<Context>| m.collect_inline_declarations(context, inline_name);
        match self {
            CanisterMethod::Update(update_method) => f(update_method),
            CanisterMethod::Query(query_method) => f(query_method),
            CanisterMethod::Init(init) => f(init),
            CanisterMethod::PreUpgrade(pre_upgrade) => f(pre_upgrade),
            CanisterMethod::PostUpgrade(post_upgrade) => f(post_upgrade),
            CanisterMethod::InspectMessage(inspect_message) => f(inspect_message),
            CanisterMethod::Heartbeat(heartbeat) => f(heartbeat),
        }
    }
}

pub fn get_type_refs(params: &Vec<Param>, return_type: Option<&ReturnType>) -> Vec<TypeRef> {
    vec![
        params
            .iter()
            .map(|param| param.candid_type.as_type_ref())
            .collect(),
        match return_type {
            Some(return_type) => {
                vec![return_type.as_type_ref()]
            }
            None => vec![],
        },
    ]
    .into_iter()
    .flatten()
    .filter_map(|f| f)
    .collect()
}
