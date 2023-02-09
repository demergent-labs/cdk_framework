use super::node::{DataType, Node};

pub trait ToNode {
    fn to_node(&self) -> Node;
}

pub trait ToDataType {
    fn to_data_type(&self) -> DataType;
}

impl<T> ToNode for T
where
    T: ToDataType,
{
    fn to_node(&self) -> Node {
        Node::DataType(self.to_data_type())
    }
}
