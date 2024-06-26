use crate::traits::{HasDefinedNames, HasTypeRefs};

use super::node::{
    candid::TypeRef,
    canister_method::{
        HeartbeatMethod, InitMethod, InspectMessageMethod, PostUpgradeMethod, PreUpgradeMethod,
        QueryMethod, UpdateMethod,
    },
};

#[derive(Clone)]
pub struct CanisterMethods {
    pub heartbeat_method: Option<HeartbeatMethod>,
    pub init_method: Option<InitMethod>,
    pub inspect_message_method: Option<InspectMessageMethod>,
    pub post_upgrade_method: Option<PostUpgradeMethod>,
    pub pre_upgrade_method: Option<PreUpgradeMethod>,
    pub query_methods: Vec<QueryMethod>,
    pub update_methods: Vec<UpdateMethod>,
}

impl CanisterMethods {
    pub fn collect_used_guard_function_names(&self) -> Vec<String> {
        self.query_methods
            .iter()
            .filter_map(|m| m.guard_function_name.clone())
            .chain(
                self.update_methods
                    .iter()
                    .filter_map(|m| m.guard_function_name.clone()),
            )
            .collect()
    }
}

impl HasDefinedNames for CanisterMethods {
    fn get_defined_names(&self) -> Vec<String> {
        self.query_methods
            .iter()
            .map(|f| f.name.clone())
            .chain(self.update_methods.iter().map(|f| f.name.clone()))
            .collect()
    }
}

impl HasTypeRefs for CanisterMethods {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.post_upgrade_method
            .iter()
            .flat_map(|m| m.get_type_refs())
            .chain(self.init_method.iter().flat_map(|m| m.get_type_refs()))
            .chain(self.update_methods.iter().flat_map(|m| m.get_type_refs()))
            .chain(self.query_methods.iter().flat_map(|m| m.get_type_refs()))
            .collect()
    }
}
