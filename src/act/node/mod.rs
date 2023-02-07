pub mod canister_method;
pub mod data_type;
pub mod declaration;
pub mod external_canister;
pub mod function_guard;

pub use data_type::DataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;
pub use function_guard::FunctionGuard;

use self::canister_method::CanisterMethod;

#[derive(Clone)]
pub enum ActNode {
    CanisterMethod(CanisterMethod),
    DataType(DataType),
    ExternalCanister(ExternalCanister),
    StableBTreeMap,
    FunctionGuard(FunctionGuard),
}
