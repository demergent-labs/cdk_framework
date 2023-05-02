use super::{
    HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
    QueryMethod, UpdateMethod,
};
use crate::{
    act::{
        node::{candid::TypeRef, AsNode, Context, Node, Param, ReturnType},
        Declaration, Declare,
    },
    traits::HasTypeRefs,
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
            .flat_map(|param| param.candid_type.get_type_refs())
            .collect(),
        match return_type {
            Some(return_type) => return_type.get_type_refs(),
            None => vec![],
        },
    ]
    .concat()
}
