pub mod fn_param;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
pub mod query_method;
pub mod update_method;

pub use fn_param::FnParam;
pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
pub use query_method::QueryMethod;
pub use update_method::UpdateMethod;

use crate::act::node::traits::{has_params::HasParams, has_return_value::HasReturnValue};

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
