use super::node::{ActNode, DataType};

pub trait Actable {
    fn to_act_node(&self) -> ActNode;
}

pub trait ToActDataType {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> DataType;
}
