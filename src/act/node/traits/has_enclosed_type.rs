use crate::act::{
    flatten_proclamation, node::data_type::DataType, proclamation::Proclaim, Declaration,
};

pub trait HasEnclosedType {
    fn get_enclosed_type(&self) -> DataType;

    fn create_enclosed_type_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
        enclosing_type: String,
    ) -> Vec<Declaration> {
        let proclamation = self.get_enclosed_type().create_proclamation(
            keyword_list,
            self.create_enclosed_type_prefix(parental_prefix, enclosing_type),
        );
        flatten_proclamation(&proclamation)
    }

    fn create_enclosed_type_prefix(
        &self,
        parental_prefix: String,
        enclosing_type: String,
    ) -> String {
        format!("{}{}EnclosedType", parental_prefix, enclosing_type)
    }
}
