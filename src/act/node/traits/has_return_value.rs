use crate::act::node::{
    candid::{
        type_annotation::{ToTypeAnnotation, TypeAnnotation},
        CandidType,
    },
    declaration::Declare,
    Declaration,
};

pub trait HasReturnValue {
    fn get_return_type(&self) -> CandidType;

    fn get_name(&self) -> String;

    fn create_return_type_annotation(&self, keyword_list: &Vec<String>) -> TypeAnnotation {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix())
    }

    fn collect_return_inline_declarations(&self, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_return_type()
            .flatten(&keyword_list, self.create_return_type_prefix())
    }

    fn create_return_type_prefix(&self) -> String {
        format!("{}ReturnType", self.get_name())
    }
}
