use crate::act::node::Member;

use super::HasDeclarableTypes;

pub trait HasMembers {
    fn get_members(&self) -> Vec<Member>;
}

impl<T> HasDeclarableTypes<Member> for T
where
    T: HasMembers,
{
    fn get_declarable_items(&self) -> Vec<Member> {
        self.get_members()
    }
}
