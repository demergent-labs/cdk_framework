pub mod act_function_guard;
pub mod canister_method;
pub mod data_type;
pub mod external_canister;

pub use act_function_guard::FunctionGuard;
pub use data_type::DataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;

use self::canister_method::CanisterMethod;

#[derive(Clone)]
pub enum ActNode {
    CanisterMethod(CanisterMethod),
    DataType(DataType),
    ExternalCanister(ExternalCanister),
    StableBTreeMap,
    FunctionGuard(FunctionGuard),
}
