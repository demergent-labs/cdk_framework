use crate::act::node::{
    data_type::{
        type_annotation::{ToTypeAnnotation, TypeAnnotation},
        DataType,
    },
    proclamation::Proclaim,
    Declaration,
};

pub trait HasReturnValue {
    fn get_return_type(&self) -> DataType;

    fn get_name(&self) -> String;

    fn create_return_type_annotation(&self, keyword_list: &Vec<String>) -> TypeAnnotation {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix())
    }

    fn collect_return_inline_declarations(&self, keyword_list: &Vec<String>) -> Vec<Declaration> {
        let proclamation = self
            .get_return_type()
            .create_proclamation(&keyword_list, self.create_return_type_prefix());

        proclamation.flatten()
    }

    fn create_return_type_prefix(&self) -> String {
        format!("{}ReturnType", self.get_name())
    }
}
