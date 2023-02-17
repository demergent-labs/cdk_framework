use proc_macro2::TokenStream;
use std::collections::HashMap;

use crate::act::{self, node::data_type::DataType, proclamation::Proclaim};

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;

    fn create_member_prefix(&self, index: usize, name: String) -> String {
        format!("{}MemberNum{}", name, index)
    }

    fn create_member_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> HashMap<String, TokenStream> {
        self.get_members()
            .iter()
            .enumerate()
            .fold(HashMap::new(), |acc, (index, member_type)| {
                let declaration = member_type.create_proclamation(
                    keyword_list,
                    self.create_member_prefix(index, name.clone()),
                );
                act::flatten_proclamation(declaration, acc)
            })
    }
}
