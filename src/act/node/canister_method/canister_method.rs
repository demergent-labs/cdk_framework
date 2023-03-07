use super::{
    HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
    QueryMethod, UpdateMethod,
};
use crate::act::node::{declaration::Declare, Declaration, NodeContext};

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

impl Declare<NodeContext> for CanisterMethod {
    fn to_declaration(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Option<Declaration> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.to_declaration(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.to_declaration(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => {
                init_method.to_declaration(context, parental_prefix)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.to_declaration(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.to_declaration(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_method) => {
                inspect_method.to_declaration(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.to_declaration(&context.cdk_name, parental_prefix)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Vec<Declaration> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.collect_inline_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.collect_inline_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => {
                init_method.collect_inline_declarations(&context, parental_prefix)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.collect_inline_declarations(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.collect_inline_declarations(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_message_method) => inspect_message_method
                .collect_inline_declarations(&context.cdk_name, parental_prefix),
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.collect_inline_declarations(&context.cdk_name, parental_prefix)
            }
        }
    }
}
