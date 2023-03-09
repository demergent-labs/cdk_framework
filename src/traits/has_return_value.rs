use crate::act::node::ReturnType;

use super::HasDeclarableTypes;

pub trait HasReturnValue {
    fn get_return_type(&self) -> ReturnType;
}

impl<T> HasDeclarableTypes<ReturnType> for T
where
    T: HasReturnValue,
{
    fn get_declarable_items(&self) -> Vec<ReturnType> {
        vec![self.get_return_type()]
    }
}
