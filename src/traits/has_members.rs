use crate::act::node::Member;

use super::HasInlineTypes;

pub trait HasMembers {
    fn get_members(&self) -> Vec<Member>;
}

impl<T> HasInlineTypes<Member> for T
where
    T: HasMembers,
{
    fn get_inline_items(&self) -> Vec<Member> {
        self.get_members()
    }
}
