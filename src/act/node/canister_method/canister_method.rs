use super::{
    HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
    QueryMethod, UpdateMethod,
};
use crate::act::{
    node::{AsNode, Context, Node},
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
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.to_declaration(&context.keyword_list, inline_name)
            }
            CanisterMethod::Query(query_method) => {
                query_method.to_declaration(&context.keyword_list, inline_name)
            }
            CanisterMethod::Init(init_method) => init_method.to_declaration(context, inline_name),
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.to_declaration(&context.cdk_name, inline_name)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.to_declaration(context, inline_name)
            }
            CanisterMethod::InspectMessage(inspect_method) => {
                inspect_method.to_declaration(&context.cdk_name, inline_name)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.to_declaration(&context.cdk_name, inline_name)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.collect_inline_declarations(&context.keyword_list, inline_name)
            }
            CanisterMethod::Query(query_method) => {
                query_method.collect_inline_declarations(&context.keyword_list, inline_name)
            }
            CanisterMethod::Init(init_method) => {
                init_method.collect_inline_declarations(&context, inline_name)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.collect_inline_declarations(&context.cdk_name, inline_name)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.collect_inline_declarations(context, inline_name)
            }
            CanisterMethod::InspectMessage(inspect_message_method) => {
                inspect_message_method.collect_inline_declarations(&context.cdk_name, inline_name)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.collect_inline_declarations(&context.cdk_name, inline_name)
            }
        }
    }
}
