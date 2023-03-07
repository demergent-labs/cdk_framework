use crate::act::node::{candid::CandidType, declaration::Declare, Declaration};

pub trait HasMembers {
    fn get_members(&self) -> Vec<CandidType>;

    fn collect_member_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> Vec<Declaration> {
        self.get_members()
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, member_type)| {
                let declarations = member_type
                    .flatten(keyword_list, self.create_member_prefix(index, name.clone()));
                vec![acc, declarations].concat()
            })
    }

    fn create_member_prefix(&self, index: usize, name: String) -> String {
        format!("{name}MemberNum{index}")
    }
}
