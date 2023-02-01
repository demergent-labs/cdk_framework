pub mod act_function_guard;
pub mod canister_method;
pub mod data_type;
pub mod external_canister;

pub use act_function_guard::ActFunctionGuard;
pub use data_type::ActDataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;

use self::canister_method::ActCanisterMethod;

#[derive(Clone)]
pub enum ActNode {
    CanisterMethod(ActCanisterMethod),
    DataType(ActDataType),
    ExternalCanister(ExternalCanister),
    StableBTreeMap,
    FunctionGuard(ActFunctionGuard),
}
