use crate::act::node::candid::TypeRef;

pub trait HasTypeRefs {
    fn get_type_refs(&self) -> Vec<TypeRef>;
}
