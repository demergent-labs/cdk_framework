pub mod act_fn_param;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
pub mod query_method;
pub mod update_method;

pub use act_fn_param::ActFnParam;
pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
pub use query_method::QueryMethod;
pub use update_method::UpdateMethod;

use super::ActDataType;

pub enum ActCanisterMethodNew {
    Update(UpdateMethod),
    Query(QueryMethod),
    Init(InitMethod),
    PreUpgrade(PreUpgradeMethod),
    PostUpgrade(PostUpgradeMethod),
    InspectMessage(InspectMessageMethod),
    Heartbeat(HeartbeatMethod),
}

trait GetAllTypes {
    fn get_all_types(&self) -> Vec<ActDataType>;
}

impl<T> GetAllTypes for Vec<T>
where
    T: GetAllTypes,
{
    fn get_all_types(&self) -> Vec<ActDataType> {
        self.iter().fold(vec![], |acc, canister_method| {
            let inline_types = canister_method.get_all_types();
            vec![acc, inline_types].concat()
        })
    }
}

impl<T> GetAllTypes for T
where
    T: HasParams,
    T: HasReturnValue,
{
    fn get_all_types(&self) -> Vec<ActDataType> {
        vec![self.get_param_types(), vec![self.get_return_type()]].concat()
    }
}

trait HasReturnValue {
    fn get_return_type(&self) -> ActDataType;
}

trait HasParams {
    fn get_param_types(&self) -> Vec<ActDataType>;
}
