use crate::{
    act::{
        node::{CandidType, Context},
        Declaration, Declare, TypeAnnotation,
    },
    traits::{HasInlineName, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub candid_type: CandidType,
}

impl ToTypeAnnotation<Context> for Member {
    fn to_type_annotation(&self, context: &Context, parent_name: String) -> TypeAnnotation {
        self.candid_type
            .to_type_annotation(context, self.get_inline_name(&parent_name))
    }
}

impl Declare<Context> for Member {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        parent_name: String,
    ) -> Vec<Declaration> {
        self.candid_type
            .flatten(context, self.get_inline_name(&parent_name))
    }
}

impl HasInlineName for Member {
    fn get_inline_name(&self, parent_name: &String) -> String {
        format!("{parent_name}_{member_name}", member_name = self.name)
    }
}
