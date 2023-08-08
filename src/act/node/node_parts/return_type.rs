use std::ops::Deref;

use crate::{
    act::node::{CandidType, Context},
    traits::{Declare, HasInlineName, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct ReturnType {
    candid_type: CandidType,
}

impl ReturnType {
    pub fn new(candid_type: CandidType) -> ReturnType {
        return ReturnType { candid_type };
    }
}

impl Deref for ReturnType {
    type Target = CandidType;

    fn deref(&self) -> &Self::Target {
        &self.candid_type
    }
}

impl HasInlineName for ReturnType {
    fn get_inline_name(&self, function_name: &String) -> String {
        format!("{function_name}ReturnType")
    }
}

impl ToTypeAnnotation<Context> for ReturnType {
    fn to_type_annotation(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> crate::act::TypeAnnotation {
        self.candid_type.to_type_annotation(
            context,
            self.get_inline_name(&function_name),
            module_name,
        )
    }
}

impl Declare<Context> for ReturnType {
    fn to_declaration(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> Option<crate::act::Declaration> {
        self.candid_type
            .to_declaration(context, self.get_inline_name(&function_name), module_name)
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> Vec<crate::act::Declaration> {
        self.candid_type.collect_inline_declarations(
            context,
            self.get_inline_name(&function_name),
            module_name,
        )
    }
}
