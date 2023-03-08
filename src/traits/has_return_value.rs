use crate::act::{node::CandidType, Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

pub trait HasReturnValue {
    fn get_return_type(&self) -> CandidType;

    fn create_return_type_annotation(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> TypeAnnotation {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix(function_name))
    }

    fn collect_return_inline_declarations(
        &self,
        function_name: &String,
        keyword_list: &Vec<String>,
    ) -> Vec<Declaration> {
        self.get_return_type()
            .flatten(&keyword_list, self.create_return_type_prefix(function_name))
    }

    fn create_return_type_prefix(&self, function_name: &String) -> String {
        format!("{function_name}ReturnType")
    }
}
