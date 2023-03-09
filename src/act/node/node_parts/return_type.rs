use std::ops::Deref;

use crate::{
    act::node::CandidType,
    traits::{Declare, HasPrefix, ToTypeAnnotation},
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

impl HasPrefix for ReturnType {
    fn get_prefix(&self, function_name: &String) -> String {
        format!("{function_name}ReturnType")
    }
}

impl ToTypeAnnotation<Vec<String>> for ReturnType {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        function_name: String,
    ) -> crate::act::TypeAnnotation {
        self.candid_type
            .to_type_annotation(keyword_list, self.get_prefix(&function_name))
    }
}

impl Declare<Vec<String>> for ReturnType {
    fn to_declaration(
        &self,
        context: &Vec<String>,
        function_name: String,
    ) -> Option<crate::act::Declaration> {
        self.candid_type
            .to_declaration(context, self.get_prefix(&function_name))
    }

    fn collect_inline_declarations(
        &self,
        context: &Vec<String>,
        function_name: String,
    ) -> Vec<crate::act::Declaration> {
        self.candid_type
            .collect_inline_declarations(context, self.get_prefix(&function_name))
    }
}
