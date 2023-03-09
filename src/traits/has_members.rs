use crate::act::node::Member;

use super::HasInlines;

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
