pub mod act;
pub mod generators;
pub mod keyword;
pub mod traits;

pub use act::AbstractCanisterTree;
pub use traits::ToDataType;

pub enum SystemStructureType {
    Canister,
}
