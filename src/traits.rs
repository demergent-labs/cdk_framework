use crate::{
    act::nodes::{
        canister_methods::{
            ActFnParam, HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod,
            PreUpgradeMethod, QueryMethod, UpdateMethod,
        },
        ActDataType,
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

// TODO what is this? Can we use it?
pub trait CanisterMethodBuilder {
    fn build_update_method_node(&self, request_type: &RequestType) -> UpdateMethod;
    fn build_query_method_node(&self, request_type: &RequestType) -> QueryMethod;
    fn build_params(&self) -> Vec<ActFnParam>;
    fn build_return_type(&self) -> ActDataType;
}
