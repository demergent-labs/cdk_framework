use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::node::{
        canister_method::{
            CanisterMethod,
            {
                HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod,
                PreUpgradeMethod, QueryMethod, UpdateMethod,
            },
        },
        data_type::{func, Func, Record, Tuple, TypeAlias, Variant},
        proclamation::Proclaim,
        DataType, Node, NodeContext, {ExternalCanister, GuardFunction},
    },
    generators::{candid_file_generation, random, vm_value_conversion},
};

mod better;
pub mod node;
mod worse;

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub canister_methods: CanisterMethods,
    pub data_types: DataTypes,
    pub external_canisters: Vec<ExternalCanister>,
    pub guard_functions: Vec<GuardFunction>,
    pub header: TokenStream,
    pub body: TokenStream,
    pub try_from_vm_value_impls: TokenStream,
    pub try_into_vm_value_impls: TokenStream,
    pub keywords: Vec<String>,
}

pub struct CanisterMethods {
    pub heartbeat_method: Option<HeartbeatMethod>,
    pub init_method: InitMethod,
    pub inspect_message_method: Option<InspectMessageMethod>,
    pub post_upgrade_method: PostUpgradeMethod,
    pub pre_upgrade_method: Option<PreUpgradeMethod>,
    pub query_methods: Vec<QueryMethod>,
    pub update_methods: Vec<UpdateMethod>,
}

pub struct DataTypes {
    pub funcs: Vec<Func>,
    pub records: Vec<Record>,
    pub tuples: Vec<Tuple>,
    pub type_aliases: Vec<TypeAlias>,
    pub variants: Vec<Variant>,
}
