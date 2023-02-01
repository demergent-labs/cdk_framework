use super::DataType;

pub trait HasMembers {
    fn get_members(&self) -> Vec<DataType>;
}
