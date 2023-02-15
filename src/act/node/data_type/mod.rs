use proc_macro2::TokenStream;
use std::collections::HashMap;

use self::traits::ToTypeAnnotation;
use crate::act::proclamation::Proclaim;

pub mod array;
pub mod boxed;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod traits;
pub mod tuple;
pub mod type_alias;
pub mod type_ref;
pub mod variant;

pub use array::Array;
pub use boxed::Boxed;
pub use func::Func;
pub use opt::Opt;
pub use primitive::Primitive;
pub use record::Record;
pub use tuple::Tuple;
pub use type_alias::TypeAlias;
pub use type_ref::TypeRef;
pub use variant::Variant;

#[derive(Clone, Debug)]
pub enum DataType {
    Array(Array),
    Boxed(Boxed),
    Func(Func),
    Opt(Opt),
    Primitive(Primitive),
    Record(Record),
    Tuple(Tuple),
    TypeAlias(TypeAlias),
    TypeRef(TypeRef),
    Variant(Variant),
}

impl DataType {
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            DataType::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn as_boxed(&self) -> Option<&Boxed> {
        match self {
            DataType::Boxed(boxed) => Some(&boxed),
            _ => None,
        }
    }

    pub fn as_func(&self) -> Option<&Func> {
        match self {
            DataType::Func(func) => Some(&func),
            _ => None,
        }
    }

    pub fn as_opt(&self) -> Option<&Opt> {
        match self {
            DataType::Opt(option) => Some(&option),
            _ => None,
        }
    }

    pub fn as_primitive(&self) -> Option<&Primitive> {
        match self {
            DataType::Primitive(primitive) => Some(&primitive),
            _ => None,
        }
    }

    pub fn as_record(&self) -> Option<&Record> {
        match self {
            DataType::Record(record) => Some(&record),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&Tuple> {
        match self {
            DataType::Tuple(tuple) => Some(&tuple),
            _ => None,
        }
    }

    pub fn as_type_alias(&self) -> Option<&TypeAlias> {
        match self {
            DataType::TypeAlias(type_alias) => Some(&type_alias),
            _ => None,
        }
    }

    pub fn as_variant(&self) -> Option<&Variant> {
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

    pub fn is_opt(&self) -> bool {
        match self {
            DataType::Opt(_) => true,
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
            DataType::Opt(opt) => opt.to_type_annotation(keyword_list, parental_prefix),
            DataType::Primitive(primitive) => {
                primitive.to_type_annotation(keyword_list, parental_prefix)
            }
            DataType::Record(record) => record.to_type_annotation(keyword_list, parental_prefix),
            DataType::Tuple(tuple) => tuple.to_type_annotation(keyword_list, parental_prefix),
            DataType::TypeAlias(type_alias) => {
                type_alias.to_type_annotation(keyword_list, parental_prefix)
            }
            DataType::TypeRef(type_ref) => {
                type_ref.to_type_annotation(keyword_list, parental_prefix)
            }
            DataType::Variant(variant) => variant.to_type_annotation(keyword_list, parental_prefix),
        }
    }
}

impl Proclaim<Vec<String>> for DataType {
    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            DataType::Array(array) => {
                array.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::Boxed(boxed) => {
                boxed.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::Func(func) => func.collect_inline_declarations(keyword_list, parental_prefix),
            DataType::Opt(opt) => opt.collect_inline_declarations(keyword_list, parental_prefix),
            DataType::Primitive(primitive) => {
                primitive.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::Record(record) => {
                record.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::Tuple(tuple) => {
                tuple.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::TypeAlias(type_alias) => {
                type_alias.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::TypeRef(type_ref) => {
                type_ref.collect_inline_declarations(keyword_list, parental_prefix)
            }
            DataType::Variant(variant) => {
                variant.collect_inline_declarations(keyword_list, parental_prefix)
            }
        }
    }

    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> std::option::Option<TokenStream> {
        let prefix = format!("DataType{}", parental_prefix);
        match self {
            DataType::Array(array) => array.create_declaration(keyword_list, parental_prefix),
            DataType::Boxed(boxed) => boxed.create_declaration(keyword_list, parental_prefix),
            DataType::Func(func) => func.create_declaration(keyword_list, parental_prefix),
            DataType::Opt(opt) => opt.create_declaration(keyword_list, parental_prefix),
            DataType::Primitive(primitive) => {
                primitive.create_declaration(keyword_list, parental_prefix)
            }
            DataType::Record(record) => record.create_declaration(keyword_list, prefix),
            DataType::Tuple(tuple) => tuple.create_declaration(keyword_list, parental_prefix),
            DataType::TypeAlias(type_alias) => type_alias.create_declaration(keyword_list, prefix),
            DataType::TypeRef(type_ref) => {
                type_ref.create_declaration(keyword_list, parental_prefix)
            }
            DataType::Variant(variant) => variant.create_declaration(keyword_list, parental_prefix),
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> std::option::Option<String> {
        match self {
            DataType::Array(array) => array.create_identifier(parental_prefix),
            DataType::Boxed(boxed) => boxed.create_identifier(parental_prefix),
            DataType::Func(func) => func.create_identifier(parental_prefix),
            DataType::Opt(opt) => opt.create_identifier(parental_prefix),
            DataType::Primitive(primitive) => primitive.create_identifier(parental_prefix),
            DataType::Record(record) => record.create_identifier(parental_prefix),
            DataType::Tuple(tuple) => tuple.create_identifier(parental_prefix),
            DataType::TypeAlias(type_alias) => type_alias.create_identifier(parental_prefix),
            DataType::TypeRef(type_ref) => type_ref.create_identifier(parental_prefix),
            DataType::Variant(variant) => variant.create_identifier(parental_prefix),
        }
    }
}
