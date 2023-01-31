pub mod act_function_guard;
pub mod canister_methods;
pub mod data_types;
pub mod external_canister;

pub use act_function_guard::ActFunctionGuard;
pub use data_types::ActDataType;
pub use external_canister::ActExternalCanister;
pub use external_canister::ExternalCanisterMethod;

use self::canister_methods::ActCanisterMethodNew;

pub enum ActNode {
    CanisterMethod(ActCanisterMethodNew),
    DataType(ActDataType),
    ExternalCanister(ActExternalCanister),
    StableBTreeMap,
    FunctionGuard(ActFunctionGuard),
}
