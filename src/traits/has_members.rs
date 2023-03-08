use crate::act::{node::CandidType, Declaration, Declare};

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub candid_type: CandidType,
}

pub trait HasMembers {
    fn get_members(&self) -> Vec<Member>;

    fn collect_member_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> Vec<Declaration> {
        self.get_members().iter().fold(vec![], |acc, member| {
            let declarations = member.candid_type.flatten(
                keyword_list,
                self.create_member_prefix(member, name.clone()),
            );
            vec![acc, declarations].concat()
        })
    }

    fn create_member_prefix(&self, member: &Member, name: String) -> String {
        format!("{name}Member{member_name}", member_name = member.name)
    }
}
