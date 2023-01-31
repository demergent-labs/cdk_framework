// use act_data_type::{build_inline_type_acts, deduplicate, ActDataType};
// use arrays::{ActArrayLiteral, ActArrayTypeAlias};
// // pub use funcs::generate_func_arg_token;
// use option::{ActOptionLiteral, ActOptionTypeAlias};
// use primitives::{ActPrimitiveLit, ActPrimitiveTypeAlias};
// use record::{ActRecordMember, Record};
// use tuple::{ActTupleElem, Tuple};
// use type_ref::{ActTypeRefLit, ActTypeRefTypeAlias};
// use variants::{ActVariantMember, Variant};

pub mod array;
pub mod func;
pub mod option;
pub mod primitive;
pub mod record;
pub mod traits;
pub mod tuple;
pub mod type_ref;
pub mod variant;

use crate::ToTokenStream;
use proc_macro2::TokenStream;
use std::collections::HashMap;

pub use array::Array;
pub use func::ActFunc;
pub use option::Option;
pub use primitive::ActPrimitive;
pub use record::ActRecord;
pub use tuple::ActTuple;
pub use type_ref::ActTypeRef;
pub use variant::ActVariant;

use self::traits::{HasMembers, Literally, TypeAliasize};

#[derive(Clone, Debug)]
pub enum DataType {
    Array(Array),
    Func(ActFunc),
    Option(self::Option),
    Primitive(ActPrimitive),
    Record(ActRecord),
    Tuple(ActTuple),
    TypeRef(ActTypeRef),
    Variant(ActVariant),
}

#[derive(Clone, Debug)]
pub enum LiteralOrTypeAlias<L, T> {
    Literal(L),
    TypeAlias(T),
}

impl DataType {
    pub fn as_array(&self) -> core::option::Option<&Array> {
        match self {
            DataType::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn as_func(&self) -> core::option::Option<&ActFunc> {
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
    pub fn as_primitive(&self) -> core::option::Option<&ActPrimitive> {
        match self {
            DataType::Primitive(primitive) => Some(&primitive),
            _ => None,
        }
    }
    pub fn as_record(&self) -> core::option::Option<&ActRecord> {
        match self {
            DataType::Record(record) => Some(&record),
            _ => None,
        }
    }
    pub fn as_tuple(&self) -> core::option::Option<&ActTuple> {
        match self {
            DataType::Tuple(tuple) => Some(&tuple),
            _ => None,
        }
    }
    pub fn as_type_ref(&self) -> core::option::Option<&ActTypeRef> {
        match self {
            DataType::TypeRef(type_ref) => Some(&type_ref),
            _ => None,
        }
    }
    pub fn as_variant(&self) -> core::option::Option<&ActVariant> {
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
    pub fn is_type_ref(&self) -> bool {
        match self {
            DataType::TypeRef(_) => true,
            _ => false,
        }
    }
    pub fn is_variant(&self) -> bool {
        match self {
            DataType::Variant(_) => true,
            _ => false,
        }
    }
    pub fn needs_definition(&self) -> bool {
        match self {
            DataType::Primitive(_) => false,
            DataType::TypeRef(_) => false,
            DataType::Array(_) => false,
            DataType::Option(_) => false,
            DataType::Record(act_record) => act_record.act_type.is_literal(),
            DataType::Variant(act_variant) => act_variant.act_type.is_literal(),
            DataType::Func(act_func) => act_func.act_type.is_literal(),
            DataType::Tuple(act_tuple) => act_tuple.act_type.is_literal(),
        }
    }

    pub fn as_type_alias(&self) -> core::option::Option<DataType> {
        match self {
            DataType::Primitive(_) => None,
            DataType::Option(_) => None,
            DataType::TypeRef(_) => None,
            DataType::Array(_) => None,
            DataType::Record(record) => Some(DataType::Record(record.as_type_alias())),
            DataType::Variant(variant) => Some(DataType::Variant(variant.as_type_alias())),
            DataType::Func(func) => Some(DataType::Func(func.as_type_alias())),
            DataType::Tuple(tuple) => Some(DataType::Tuple(tuple.as_type_alias())),
        }
    }

    pub fn needs_to_be_boxed(&self) -> bool {
        true
    }

    pub fn get_members(&self) -> Vec<DataType> {
        match self {
            DataType::Record(act_record) => act_record.get_members(),
            DataType::Variant(act_variant) => act_variant.get_members(),
            DataType::Func(act_func) => act_func.get_members(),
            DataType::Primitive(_) => vec![],
            DataType::TypeRef(_) => vec![],
            DataType::Array(act_array) => act_array.get_members(),
            DataType::Tuple(act_tuple) => act_tuple.get_members(),
            DataType::Option(act_option) => act_option.get_members(),
        }
    }

    pub fn collect_inline_types(&self) -> Vec<DataType> {
        let act_data_type = match self.needs_definition() {
            true => match self.as_type_alias() {
                Some(type_alias) => vec![type_alias],
                None => vec![],
            },
            false => vec![],
        };
        let member_act_data_types = self.get_members();
        let all_descendant_act_data_types =
            member_act_data_types.iter().fold(vec![], |acc, member| {
                vec![acc, member.collect_inline_types()].concat()
            });
        vec![act_data_type, all_descendant_act_data_types].concat()
    }
}

impl ToTokenStream<&Vec<String>> for DataType {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        match self {
            DataType::Record(act_record) => act_record.to_token_stream(keyword_list),
            DataType::Variant(act_variant) => act_variant.to_token_stream(keyword_list),
            DataType::Func(act_func) => act_func.to_token_stream(keyword_list),
            DataType::Tuple(act_tuple) => act_tuple.to_token_stream(keyword_list),
            DataType::Primitive(act_primitive) => act_primitive.to_token_stream(keyword_list),
            DataType::TypeRef(act_type_ref) => act_type_ref.to_token_stream(keyword_list),
            DataType::Option(act_option) => act_option.to_token_stream(keyword_list),
            DataType::Array(act_array) => act_array.to_token_stream(keyword_list),
        }
    }
}

pub fn build_inline_type_acts(type_aliases: &Vec<DataType>) -> Vec<DataType> {
    type_aliases.iter().fold(vec![], |acc, type_alias| {
        vec![acc, type_alias.collect_inline_types()].concat()
    })
}

pub fn deduplicate(
    act_data_type_nodes: Vec<DataType>,
    keyword_list: &Vec<String>,
) -> Vec<DataType> {
    let map: HashMap<String, DataType> =
        act_data_type_nodes
            .iter()
            .fold(HashMap::new(), |mut acc, act_node| {
                match acc.get(&act_node.to_token_stream(keyword_list).to_string()) {
                    Some(_) => acc,
                    None => {
                        acc.insert(
                            act_node.to_token_stream(keyword_list).to_string(),
                            act_node.clone(),
                        );
                        acc
                    }
                }
            });
    map.values().cloned().collect()
}
