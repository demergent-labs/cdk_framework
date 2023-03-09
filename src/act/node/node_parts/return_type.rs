use crate::{act::node::CandidType, traits::HasPrefix};

pub type ReturnType = CandidType;

impl HasPrefix for ReturnType {
    fn get_prefix(&self, function_name: &String) -> String {
        format!("{function_name}ReturnType")
    }
}
