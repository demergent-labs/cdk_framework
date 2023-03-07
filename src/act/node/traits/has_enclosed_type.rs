use crate::act::node::{candid::CandidType, declaration::Declare, Declaration};

pub trait HasEnclosedType {
    fn get_enclosed_type(&self) -> CandidType;

    fn collect_enclosed_type_inline_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
        enclosing_type: String,
    ) -> Vec<Declaration> {
        self.get_enclosed_type().flatten(
            keyword_list,
            self.create_enclosed_type_prefix(parental_prefix, enclosing_type),
        )
    }

    fn create_enclosed_type_prefix(
        &self,
        parental_prefix: String,
        enclosing_type: String,
    ) -> String {
        format!("{}{}EnclosedType", parental_prefix, enclosing_type)
    }
}
