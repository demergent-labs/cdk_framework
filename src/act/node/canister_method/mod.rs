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

use super::DataType;

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

pub trait GetAllTypes {
    fn get_all_types(&self) -> Vec<DataType>;
}

impl<T> GetAllTypes for Vec<T>
where
    T: GetAllTypes,
{
    fn get_all_types(&self) -> Vec<DataType> {
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
    fn get_all_types(&self) -> Vec<DataType> {
        vec![self.get_param_types(), vec![self.get_return_type()]].concat()
    }
}

pub trait HasReturnValue {
    fn get_return_type(&self) -> DataType;
}

pub trait HasParams {
    fn get_param_types(&self) -> Vec<DataType>;
}

pub trait HasName {
    fn get_name(&self) -> String;
}
