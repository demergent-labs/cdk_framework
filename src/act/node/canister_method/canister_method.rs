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
        let to_declaration = |m: &dyn Declare<Context>| m.to_declaration(context, inline_name);
        match self {
            CanisterMethod::Update(update_method) => to_declaration(update_method),
            CanisterMethod::Query(query_method) => to_declaration(query_method),
            CanisterMethod::Init(init) => to_declaration(init),
            CanisterMethod::PreUpgrade(pre_upgrade) => to_declaration(pre_upgrade),
            CanisterMethod::PostUpgrade(post_upgrade) => to_declaration(post_upgrade),
            CanisterMethod::InspectMessage(inspect_message) => to_declaration(inspect_message),
            CanisterMethod::Heartbeat(heartbeat) => to_declaration(heartbeat),
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        let collect_inline_declarations =
            |m: &dyn Declare<Context>| m.collect_inline_declarations(context, inline_name);
        match self {
            CanisterMethod::Update(update_method) => collect_inline_declarations(update_method),
            CanisterMethod::Query(query_method) => collect_inline_declarations(query_method),
            CanisterMethod::Init(init) => collect_inline_declarations(init),
            CanisterMethod::PreUpgrade(pre_upgrade) => collect_inline_declarations(pre_upgrade),
            CanisterMethod::PostUpgrade(post_upgrade) => collect_inline_declarations(post_upgrade),
            CanisterMethod::InspectMessage(inspect_message) => {
                collect_inline_declarations(inspect_message)
            }
            CanisterMethod::Heartbeat(heartbeat) => collect_inline_declarations(heartbeat),
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
