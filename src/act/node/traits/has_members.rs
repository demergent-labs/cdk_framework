use std::collections::HashMap;

use proc_macro2::TokenStream;

use crate::act::declaration::ToDeclaration;

use crate::act::node::data_type::DataType;

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;
    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String;
    fn create_member_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.get_members().iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (index, member_type)| {
                let declaration = member_type.create_declaration(
                    keyword_list,
                    self.create_member_prefix(index, parental_prefix.clone()),
                );

                if let Some(identifier) = &declaration.identifier {
                    if let Some(code) = declaration.code {
                        acc.insert(identifier.clone(), code);
                    }
                }
                acc.extend(declaration.children.clone().into_iter());
                acc
            },
        )
    }
}
