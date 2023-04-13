use crate::act::node::{candid::TypeRef, Member};

use super::{HasInlines, HasTypeRefs};

pub trait HasMembers {
    fn get_members(&self) -> Vec<Member>;
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
        self.get_members()
            .iter()
            .filter_map(|member| member.candid_type.as_type_ref())
            .collect()
    }
}
