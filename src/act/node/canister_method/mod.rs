pub mod fn_param;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
pub mod query_method;
pub mod update_method;

use std::collections::HashMap;

pub use fn_param::FnParam;
pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
use proc_macro2::TokenStream;
pub use query_method::QueryMethod;
pub use update_method::UpdateMethod;

use crate::act::declaration::ToDeclaration;

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

#[derive(Clone)]
pub struct CanisterMethodContext {
    pub keyword_list: Vec<String>,
    pub cdk_name: String,
}

impl ToDeclaration<CanisterMethodContext> for CanisterMethod {
    fn create_code(
        &self,
        context: &CanisterMethodContext,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.create_code(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.create_code(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => init_method.create_code(context, parental_prefix),
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.create_code(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.create_code(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_method) => {
                inspect_method.create_code(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.create_code(&context.cdk_name, parental_prefix)
            }
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.create_identifier(parental_prefix)
            }
            CanisterMethod::Query(query_method) => query_method.create_identifier(parental_prefix),
            CanisterMethod::Init(init_method) => init_method.create_identifier(parental_prefix),
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.create_identifier(parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.create_identifier(parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_message) => {
                inspect_message.create_identifier(parental_prefix)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.create_identifier(parental_prefix)
            }
        }
    }

    fn create_child_declarations(
        &self,
        context: &CanisterMethodContext,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.create_child_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.create_child_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => {
                init_method.create_child_declarations(&context, parental_prefix)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.create_child_declarations(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.create_child_declarations(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_message_method) => {
                inspect_message_method.create_child_declarations(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.create_child_declarations(&context.cdk_name, parental_prefix)
            }
        }
    }
}
