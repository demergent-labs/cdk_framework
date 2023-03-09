use crate::act::{node::ReturnType, ToTypeAnnotation, TypeAnnotation};

use super::{HasDeclarableTypes, HasPrefix};

pub trait HasReturnValue {
    fn get_return_type(&self) -> ReturnType;

    fn create_return_type_annotation(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> TypeAnnotation {
        let return_type = self.get_return_type();
        return_type.to_type_annotation(keyword_list, return_type.get_prefix(function_name))
    }
}

impl<T> HasDeclarableTypes<ReturnType> for T
where
    T: HasReturnValue,
{
    fn get_declarable_items(&self) -> Vec<ReturnType> {
        vec![self.get_return_type()]
    }
}
