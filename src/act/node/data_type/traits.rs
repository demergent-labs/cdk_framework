use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::act::node::full_declaration::{Declaration, ToDeclaration};

use super::DataType;

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;
    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String;
    fn create_member_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, Declaration> {
        self.get_members().iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (index, member_type)| {
                let declaration = member_type.create_declaration(
                    keyword_list,
                    self.create_member_prefix(index, parental_prefix.clone()),
                );
                acc.extend(declaration.children.clone().into_iter());
                if let Some(identifier) = &declaration.identifier {
                    if let Some(_) = declaration.code {
                        acc.insert(identifier.clone(), declaration);
                    }
                }
                acc
            },
        )
    }
}

pub trait ToTypeAnnotation<C> {
    fn to_type_annotation(&self, context: &C, parental_prefix: String) -> TokenStream;
}
