use proc_macro2::TokenStream;
use std::collections::HashMap;

use self::traits::ToTypeAnnotation;
use crate::act::declaration::ToDeclaration;

pub mod array;
pub mod boxed;
pub mod func;
pub mod option;
pub mod primitive;
pub mod record;
pub mod traits;
pub mod tuple;
pub mod type_alias;
pub mod variant;

pub use array::Array;
pub use boxed::Boxed;
pub use func::Func;
pub use option::Option;
pub use primitive::Primitive;
pub use record::Record;
pub use tuple::Tuple;
pub use type_alias::TypeAlias;
pub use variant::Variant;

#[derive(Clone, Debug)]
pub enum DataType {
    Array(Array),
    Boxed(Boxed),
    Func(Func),
    Option(self::Option),
    Primitive(Primitive),
    Record(Record),
    Tuple(Tuple),
    TypeAlias(TypeAlias),
    Variant(Variant),
}

impl DataType {
    pub fn as_array(&self) -> core::option::Option<&Array> {
        match self {
            DataType::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn as_boxed(&self) -> core::option::Option<&Boxed> {
        match self {
            DataType::Boxed(boxed) => Some(&boxed),
            _ => None,
        }
    }

    pub fn as_func(&self) -> core::option::Option<&Func> {
        match self {
            DataType::Func(func) => Some(&func),
            _ => None,
        }
    }

    pub fn as_option(&self) -> core::option::Option<&self::Option> {
        match self {
            DataType::Option(option) => Some(&option),
            _ => None,
        }
    }

    pub fn as_primitive(&self) -> core::option::Option<&Primitive> {
        match self {
            DataType::Primitive(primitive) => Some(&primitive),
            _ => None,
        }
    }

    pub fn as_record(&self) -> core::option::Option<&Record> {
        match self {
            DataType::Record(record) => Some(&record),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> core::option::Option<&Tuple> {
        match self {
            DataType::Tuple(tuple) => Some(&tuple),
            _ => None,
        }
    }

    pub fn as_type_alias(&self) -> core::option::Option<&TypeAlias> {
        match self {
            DataType::TypeAlias(type_alias) => Some(&type_alias),
            _ => None,
        }
    }

    pub fn as_variant(&self) -> core::option::Option<&Variant> {
        match self {
            DataType::Variant(variant) => Some(&variant),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            DataType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_boxed(&self) -> bool {
        match self {
            DataType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self {
            DataType::Func(_) => true,
            _ => false,
        }
    }

    pub fn is_option(&self) -> bool {
        match self {
            DataType::Option(_) => true,
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            DataType::Primitive(_) => true,
            _ => false,
        }
    }

    pub fn is_record(&self) -> bool {
        match self {
            DataType::Record(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            DataType::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_variant(&self) -> bool {
        match self {
            DataType::Variant(_) => true,
            _ => false,
        }
    }
}

impl ToTypeAnnotation<Vec<String>> for DataType {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> TokenStream {
        match self {
            DataType::Array(array) => array.to_type_annotation(keyword_list, parental_prefix),
            DataType::Boxed(boxed) => boxed.to_type_annotation(keyword_list, parental_prefix),
            DataType::Func(func) => func.to_type_annotation(keyword_list, parental_prefix),
            DataType::Option(option) => option.to_type_annotation(keyword_list, parental_prefix),
            DataType::Primitive(primitive) => {
                primitive.to_type_annotation(keyword_list, parental_prefix)
            }
            DataType::Record(record) => record.to_type_annotation(keyword_list, parental_prefix),
            DataType::Tuple(tuple) => tuple.to_type_annotation(keyword_list, parental_prefix),
            DataType::TypeAlias(type_alias) => {
                type_alias.to_type_annotation(keyword_list, parental_prefix)
            }
            DataType::Variant(variant) => variant.to_type_annotation(keyword_list, parental_prefix),
        }
    }
}

impl ToDeclaration<Vec<String>> for DataType {
    fn create_child_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            DataType::Array(array) => {
                array.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Boxed(boxed) => {
                boxed.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Func(func) => func.create_child_declarations(keyword_list, parental_prefix),
            DataType::Option(option) => {
                option.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Primitive(primitive) => {
                primitive.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Record(record) => {
                record.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Tuple(tuple) => {
                tuple.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::TypeAlias(type_alias) => {
                type_alias.create_child_declarations(keyword_list, parental_prefix)
            }
            DataType::Variant(variant) => {
                variant.create_child_declarations(keyword_list, parental_prefix)
            }
        }
    }

    fn create_code(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> std::option::Option<TokenStream> {
        let prefix = format!("DataType{}", parental_prefix);
        match self {
            DataType::Array(array) => array.create_code(keyword_list, parental_prefix),
            DataType::Boxed(boxed) => boxed.create_code(keyword_list, parental_prefix),
            DataType::Func(func) => func.create_code(keyword_list, parental_prefix),
            DataType::Option(option) => option.create_code(keyword_list, parental_prefix),
            DataType::Primitive(primitive) => primitive.create_code(keyword_list, parental_prefix),
            DataType::Record(record) => record.create_code(keyword_list, prefix),
            DataType::Tuple(tuple) => tuple.create_code(keyword_list, parental_prefix),
            DataType::TypeAlias(type_alias) => type_alias.create_code(keyword_list, prefix),
            DataType::Variant(variant) => variant.create_code(keyword_list, parental_prefix),
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> std::option::Option<String> {
        match self {
            DataType::Array(array) => array.create_identifier(parental_prefix),
            DataType::Boxed(boxed) => boxed.create_identifier(parental_prefix),
            DataType::Func(func) => func.create_identifier(parental_prefix),
            DataType::Option(option) => option.create_identifier(parental_prefix),
            DataType::Primitive(primitive) => primitive.create_identifier(parental_prefix),
            DataType::Record(record) => record.create_identifier(parental_prefix),
            DataType::Tuple(tuple) => tuple.create_identifier(parental_prefix),
            DataType::TypeAlias(type_alias) => type_alias.create_identifier(parental_prefix),
            DataType::Variant(variant) => variant.create_identifier(parental_prefix),
        }
    }
}
