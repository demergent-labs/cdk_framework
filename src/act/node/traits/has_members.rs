use crate::act::node::{data_type::DataType, proclamation::Proclaim, Declaration};

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;

    fn collect_member_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> Vec<Declaration> {
        self.get_members()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, member_type)| {
                let proclamation = member_type.create_proclamation(
                    keyword_list,
                    self.create_member_prefix(index, name.clone()),
                );
                vec![acc, proclamation.flatten()].concat()
            })
    }

    fn create_member_prefix(&self, index: usize, name: String) -> String {
        format!("{}MemberNum{}", name, index)
    }
}
