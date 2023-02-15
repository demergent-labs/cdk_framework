use proc_macro2::Ident;
use quote::format_ident;

use crate::act::node::{
    canister_method::{
        HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
        QueryMethod, UpdateMethod,
    },
    param::Param,
    DataType,
};

pub trait SystemCanisterMethodBuilder {
    fn build_heartbeat_method(&self) -> Option<HeartbeatMethod>;
    fn build_init_method(&self) -> InitMethod;
    fn build_inspect_method(&self) -> Option<InspectMessageMethod>;
    fn build_pre_upgrade_method(&self) -> PreUpgradeMethod;
    fn build_post_upgrade_method(&self) -> PostUpgradeMethod;
}

// TODO this got a little weird after we split Query and Update.
pub trait CanisterMethodBuilder {
    fn build_request_method_node(&self, request_type: &RequestType) -> RequestNode;
    fn build_params(&self) -> Vec<Param>;
    fn build_return_type(&self) -> DataType;
}

// TODO I made this as a hack to make CanisterMethodBuilder to still work. If we
// decide we want to continue down this path then i would move this to live with
// update and query inside of canister_method mod
pub enum RequestNode {
    Query(QueryMethod),
    Update(UpdateMethod),
}

pub enum RequestType {
    Query,
    Update,
}

pub trait ToIdent {
    fn to_identifier(&self) -> Ident;
}

impl ToIdent for String {
    fn to_identifier(&self) -> Ident {
        format_ident!("{}", self)
    }
}
