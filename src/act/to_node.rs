use super::node::{DataType, Node};

pub trait ToNode {
    fn to_node(&self, found_types: Vec<String>) -> Node;
}

pub trait ToDataType {
    fn to_data_type(&self, found_types: Vec<String>) -> (DataType, Vec<String>);
}

impl<T> ToNode for T
where
    T: ToDataType,
{
    fn to_node(&self, found_types: Vec<String>) -> Node {
        Node::DataType(self.to_data_type(found_types).0)
    }
}
