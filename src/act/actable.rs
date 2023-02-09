use super::node::{DataType, Node};

pub trait Actable {
    fn to_act_node(&self) -> Node;
}

pub trait ToActDataType {
    fn to_act_data_type(&self, alias_name: &Option<&String>) -> DataType;
}
