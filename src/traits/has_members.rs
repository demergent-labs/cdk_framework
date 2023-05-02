use crate::act::node::{
    candid::{type_param::TypeParams, TypeRef},
    Member,
};

use super::{HasInlines, HasTypeRefs};

pub trait HasMembers {
    fn get_members(&self) -> Vec<Member>;
    fn get_type_params(&self) -> TypeParams;
}

impl<T> HasInlines<Member> for T
where
    T: HasMembers,
{
    fn get_inlines(&self) -> Vec<Member> {
        self.get_members()
    }
}

impl<T> HasTypeRefs for T
where
    T: HasMembers,
{
    fn get_type_refs(&self) -> Vec<TypeRef> {
        let type_ref_names: Vec<_> = self
            .get_type_params()
            .iter()
            .map(|tp| tp.name.clone())
            .collect();
        // Return all of the type refs that aren't defined by the type params
        self.get_members()
            .iter()
            .filter_map(|member| member.candid_type.as_type_ref())
            .filter(|member| !type_ref_names.contains(&member.name))
            .collect()
    }
}
