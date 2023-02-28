use proc_macro2::TokenStream;
use quote::quote;
use std::fmt;

pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
pub mod query_method;
pub mod update_method;

pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
pub use query_method::QueryMethod;
pub use update_method::UpdateMethod;

use crate::{act::node::traits::HasReturnValue, traits::ToIdent};

use super::{proclamation::Proclaim, traits::HasParams, DataType, Declaration, NodeContext, Param};

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

#[derive(Clone, PartialEq)]
pub enum CanisterMethodType {
    Heartbeat,
    Init,
    InspectMessage,
    PostUpgrade,
    PreUpgrade,
    Query,
    Update,
}

#[derive(Clone, Debug)]
pub struct QueryOrUpdateDefinition {
    pub body: TokenStream,
    pub params: Vec<Param>,
    pub is_manual: bool,
    pub is_async: bool,
    pub name: String,
    pub return_type: DataType,
    pub cdk_name: String,
    pub guard_function_name: Option<String>,
}

impl HasParams for QueryOrUpdateDefinition {
    fn get_params(&self) -> Vec<Param> {
        self.params.clone()
    }
}

impl HasReturnValue for QueryOrUpdateDefinition {
    fn get_return_type(&self) -> DataType {
        self.return_type.clone()
    }
}

impl QueryOrUpdateDefinition {
    fn generate_function_body(&self, keyword_list: &Vec<String>) -> TokenStream {
        let function_name = self.name.to_identifier();
        let params = self.create_parameter_list_token_stream(keyword_list, &self.name);

        let function_body = &self.body;

        let return_type_token = self.create_return_type_annotation(keyword_list, &self.name);
        let wrapped_return_type = if self.is_manual || (self.is_async && self.cdk_name != "kybra") {
            quote! {
                ic_cdk::api::call::ManualReply<#return_type_token>
            }
        } else {
            return_type_token
        };

        quote! {
            async fn #function_name(#params) -> #wrapped_return_type {
                #function_body
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum QueryOrUpdateMethod {
    Query(QueryMethod),
    Update(UpdateMethod),
}

impl std::ops::Deref for QueryOrUpdateMethod {
    type Target = QueryOrUpdateDefinition;

    fn deref(&self) -> &Self::Target {
        match self {
            QueryOrUpdateMethod::Query(query) => &query.definition,
            QueryOrUpdateMethod::Update(update) => &update.definition,
        }
    }
}

impl fmt::Display for CanisterMethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CanisterMethodType::Heartbeat => write!(f, "Heartbeat"),
            CanisterMethodType::Init => write!(f, "Init"),
            CanisterMethodType::InspectMessage => write!(f, "InspectMessage"),
            CanisterMethodType::PostUpgrade => write!(f, "PostUpgrade"),
            CanisterMethodType::PreUpgrade => write!(f, "PreUpgrade"),
            CanisterMethodType::Query => write!(f, "Query"),
            CanisterMethodType::Update => write!(f, "Update"),
        }
    }
}

impl Proclaim<NodeContext> for CanisterMethod {
    fn create_declaration(
        &self,
        context: &NodeContext,
        parental_prefix: String,
    ) -> Option<Declaration> {
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
