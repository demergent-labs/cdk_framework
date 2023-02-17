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

    fn create_return_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        name: &String,
    ) -> TypeAnnotation {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix(name))
    }

    fn collect_return_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: &String,
    ) -> Vec<Declaration> {
        let proclamation = self
            .get_return_type()
            .create_proclamation(&keyword_list, self.create_return_type_prefix(name));

        proclamation.flatten()
    }

    fn create_return_type_prefix(&self, parental_prefix: &String) -> String {
        format!("{}ReturnType", parental_prefix)
    }
}
