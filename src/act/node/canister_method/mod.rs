use proc_macro2::TokenStream;
use std::collections::HashMap;

use crate::act::proclamation::Proclaim;

mod public_canister_methods;

pub mod fn_param;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;

pub use fn_param::FnParam;
pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
pub use public_canister_methods::query_method::QueryMethod;
pub use public_canister_methods::update_method::UpdateMethod;

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

impl Proclaim<CanisterMethodContext> for CanisterMethod {
    fn create_declaration(
        &self,
        context: &CanisterMethodContext,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.create_declaration(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.create_declaration(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => {
                init_method.create_declaration(context, parental_prefix)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.create_declaration(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.create_declaration(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_method) => {
                inspect_method.create_declaration(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.create_declaration(&context.cdk_name, parental_prefix)
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

    fn create_inline_declarations(
        &self,
        context: &CanisterMethodContext,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            CanisterMethod::Update(update_method) => {
                update_method.create_inline_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Query(query_method) => {
                query_method.create_inline_declarations(&context.keyword_list, parental_prefix)
            }
            CanisterMethod::Init(init_method) => {
                init_method.create_inline_declarations(&context, parental_prefix)
            }
            CanisterMethod::PreUpgrade(pre_upgrade_method) => {
                pre_upgrade_method.create_inline_declarations(&context.cdk_name, parental_prefix)
            }
            CanisterMethod::PostUpgrade(post_upgrade_method) => {
                post_upgrade_method.create_inline_declarations(context, parental_prefix)
            }
            CanisterMethod::InspectMessage(inspect_message_method) => inspect_message_method
                .create_inline_declarations(&context.cdk_name, parental_prefix),
            CanisterMethod::Heartbeat(heartbeat_method) => {
                heartbeat_method.create_inline_declarations(&context.cdk_name, parental_prefix)
            }
        }
    }
}
