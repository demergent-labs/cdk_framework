use crate::act::{self, node::data_type::DataType, proclamation::Proclaim, Declaration};

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;

    fn create_member_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> Vec<Declaration> {
        self.get_members()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, member_type)| {
                let declaration = member_type.create_proclamation(
                    keyword_list,
                    self.create_member_prefix(index, name.clone()),
                );
                vec![acc, act::flatten_proclamation(&declaration)].concat()
            })
    }

    fn create_member_prefix(&self, index: usize, name: String) -> String {
        format!("{}MemberNum{}", name, index)
    }
}
