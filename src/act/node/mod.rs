use self::canister_method::CanisterMethod;

pub mod canister_method;
pub mod data_type;
pub mod external_canister;
pub mod function_guard;
pub mod traits;

pub use data_type::DataType;
pub use external_canister::ExternalCanister;
pub use external_canister::ExternalCanisterMethod;
pub use function_guard::FunctionGuard;

#[derive(Clone)]
pub enum Node {
    CanisterMethod(CanisterMethod),
    DataType(DataType),
    ExternalCanister(ExternalCanister),
    StableBTreeMap,
    FunctionGuard(FunctionGuard),
}
