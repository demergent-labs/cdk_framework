use proc_macro2::Ident;
use quote::format_ident;

use crate::{
    act::node::{
        canister_method::{
            FnParam, HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod,
            PreUpgradeMethod, QueryMethod, UpdateMethod,
        },
        DataType,
    },
    RequestType,
};

pub trait SystemCanisterMethodBuilder {
    fn build_heartbeat_method(&self) -> Option<HeartbeatMethod>;
    fn build_init_method(&self) -> InitMethod;
    fn build_inspect_method(&self) -> Option<InspectMessageMethod>;
    fn build_pre_upgrade_method(&self) -> PreUpgradeMethod;
    fn build_post_upgrade_method(&self) -> PostUpgradeMethod;
}

pub trait CanisterMethodBuilder {
    fn build_update_method_node(&self, request_type: &RequestType) -> UpdateMethod;
    fn build_query_method_node(&self, request_type: &RequestType) -> QueryMethod;
    fn build_params(&self) -> Vec<FnParam>;
    fn build_return_type(&self) -> DataType;
}

pub trait ToIdent {
    fn to_identifier(&self) -> Ident;
}

impl ToIdent for String {
    fn to_identifier(&self) -> Ident {
        format_ident!("{}", self)
    }
}
