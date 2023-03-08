use crate::act::{Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

pub mod array;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod tuple;
pub mod type_alias;
pub mod type_ref;
pub mod variant;

pub use array::Array;
pub use func::Func;
pub use opt::Opt;
pub use primitive::Primitive;
pub use record::Record;
pub use tuple::Tuple;
pub use type_alias::TypeAlias;
pub use type_ref::TypeRef;
pub use variant::Variant;

use super::{AsNode, Node};

#[derive(Clone, Debug)]
pub enum CandidType {
    Array(Array),
    Func(Func),
    Opt(Opt),
    Primitive(Primitive),
    Record(Record),
    Tuple(Tuple),
    TypeAlias(TypeAlias),
    TypeRef(TypeRef),
    Variant(Variant),
}

impl AsNode for CandidType {
    fn as_node(self) -> Node {
        Node::CandidType(self)
    }
}

impl CandidType {
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            CandidType::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn as_func(&self) -> Option<&Func> {
        match self {
            CandidType::Func(func) => Some(&func),
            _ => None,
        }
    }

    pub fn as_opt(&self) -> Option<&Opt> {
        match self {
            CandidType::Opt(option) => Some(&option),
            _ => None,
        }
    }

    pub fn as_primitive(&self) -> Option<&Primitive> {
        match self {
            CandidType::Primitive(primitive) => Some(&primitive),
            _ => None,
        }
    }

    pub fn as_record(&self) -> Option<&Record> {
        match self {
            CandidType::Record(record) => Some(&record),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&Tuple> {
        match self {
            CandidType::Tuple(tuple) => Some(&tuple),
            _ => None,
        }
    }

    pub fn as_type_alias(&self) -> Option<&TypeAlias> {
        match self {
            CandidType::TypeAlias(type_alias) => Some(&type_alias),
            _ => None,
        }
    }

    pub fn as_variant(&self) -> Option<&Variant> {
        match self {
            CandidType::Variant(variant) => Some(&variant),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            CandidType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_boxed(&self) -> bool {
        match self {
            CandidType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self {
            CandidType::Func(_) => true,
            _ => false,
        }
    }

    pub fn is_opt(&self) -> bool {
        match self {
            CandidType::Opt(_) => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            CandidType::Primitive(_) => true,
            _ => false,
        }
    }

    pub fn is_record(&self) -> bool {
        match self {
            CandidType::Record(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            CandidType::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_variant(&self) -> bool {
        match self {
            CandidType::Variant(_) => true,
            _ => false,
        }
    }
}

impl ToTypeAnnotation<Vec<String>> for CandidType {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> TypeAnnotation {
        match self {
            CandidType::Array(array) => array.to_type_annotation(keyword_list, inline_name),
            CandidType::Func(func) => func.to_type_annotation(keyword_list, inline_name),
            CandidType::Opt(opt) => opt.to_type_annotation(keyword_list, inline_name),
            CandidType::Primitive(primitive) => {
                primitive.to_type_annotation(keyword_list, inline_name)
            }
            CandidType::Record(record) => record.to_type_annotation(keyword_list, inline_name),
            CandidType::Tuple(tuple) => tuple.to_type_annotation(keyword_list, inline_name),
            CandidType::TypeAlias(type_alias) => {
                type_alias.to_type_annotation(keyword_list, inline_name)
            }
            CandidType::TypeRef(type_ref) => type_ref.to_type_annotation(keyword_list, inline_name),
            CandidType::Variant(variant) => variant.to_type_annotation(keyword_list, inline_name),
        }
    }
}

impl Declare<Vec<String>> for CandidType {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Option<Declaration> {
        match self {
            CandidType::Array(array) => array.to_declaration(keyword_list, inline_name),
            CandidType::Func(func) => func.to_declaration(keyword_list, inline_name),
            CandidType::Opt(opt) => opt.to_declaration(keyword_list, inline_name),
            CandidType::Primitive(primitive) => primitive.to_declaration(keyword_list, inline_name),
            CandidType::Record(record) => record.to_declaration(keyword_list, inline_name),
            CandidType::Tuple(tuple) => tuple.to_declaration(keyword_list, inline_name),
            CandidType::TypeAlias(type_alias) => {
                type_alias.to_declaration(keyword_list, inline_name)
            }
            CandidType::TypeRef(type_ref) => type_ref.to_declaration(keyword_list, inline_name),
            CandidType::Variant(variant) => variant.to_declaration(keyword_list, inline_name),
        }
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Vec<Declaration> {
        match self {
            CandidType::Array(array) => {
                array.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::Func(func) => func.collect_inline_declarations(keyword_list, inline_name),
            CandidType::Opt(opt) => opt.collect_inline_declarations(keyword_list, inline_name),
            CandidType::Primitive(primitive) => {
                primitive.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::Record(record) => {
                record.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::Tuple(tuple) => {
                tuple.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::TypeAlias(type_alias) => {
                type_alias.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::TypeRef(type_ref) => {
                type_ref.collect_inline_declarations(keyword_list, inline_name)
            }
            CandidType::Variant(variant) => {
                variant.collect_inline_declarations(keyword_list, inline_name)
            }
        }
    }
}
