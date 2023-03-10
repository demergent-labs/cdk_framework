use crate::{
    act::{node::CandidType, Declaration, Declare, TypeAnnotation},
    traits::{HasPrefix, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub candid_type: CandidType,
}

impl ToTypeAnnotation<Vec<String>> for Member {
    fn to_type_annotation(&self, context: &Vec<String>, parent_name: String) -> TypeAnnotation {
        self.candid_type
            .to_type_annotation(context, self.get_prefix(&parent_name))
    }
}

impl Declare<Vec<String>> for Member {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Vec<String>,
        parent_name: String,
    ) -> Vec<Declaration> {
        self.candid_type
            .flatten(context, self.get_prefix(&parent_name))
    }
}

impl HasPrefix for Member {
    fn get_prefix(&self, parent_name: &String) -> String {
        format!("{parent_name}_{member_name}", member_name = self.name)
    }
}
