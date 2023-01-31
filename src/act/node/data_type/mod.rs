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

pub use array::ActArray;
pub use func::ActFunc;
pub use option::ActOption;
pub use primitive::ActPrimitive;
pub use record::ActRecord;
pub use tuple::ActTuple;
pub use type_ref::ActTypeRef;
pub use variant::ActVariant;

use self::traits::{HasMembers, Literally, TypeAliasize};

#[derive(Clone, Debug)]
pub enum ActDataType {
    Array(ActArray),
    Func(ActFunc),
    Option(ActOption),
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

impl ActDataType {
    pub fn as_array(&self) -> Option<&ActArray> {
        match self {
            ActDataType::Array(array) => Some(&array),
            _ => None,
        }
    }

    pub fn as_func(&self) -> Option<&ActFunc> {
        match self {
            ActDataType::Func(func) => Some(&func),
            _ => None,
        }
    }

    pub fn as_option(&self) -> Option<&ActOption> {
        match self {
            ActDataType::Option(option) => Some(&option),
            _ => None,
        }
    }
    pub fn as_primitive(&self) -> Option<&ActPrimitive> {
        match self {
            ActDataType::Primitive(primitive) => Some(&primitive),
            _ => None,
        }
    }
    pub fn as_record(&self) -> Option<&ActRecord> {
        match self {
            ActDataType::Record(record) => Some(&record),
            _ => None,
        }
    }
    pub fn as_tuple(&self) -> Option<&ActTuple> {
        match self {
            ActDataType::Tuple(tuple) => Some(&tuple),
            _ => None,
        }
    }
    pub fn as_type_ref(&self) -> Option<&ActTypeRef> {
        match self {
            ActDataType::TypeRef(type_ref) => Some(&type_ref),
            _ => None,
        }
    }
    pub fn as_variant(&self) -> Option<&ActVariant> {
        match self {
            ActDataType::Variant(variant) => Some(&variant),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            ActDataType::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self {
            ActDataType::Func(_) => true,
            _ => false,
        }
    }

    pub fn is_option(&self) -> bool {
        match self {
            ActDataType::Option(_) => true,
            _ => false,
        }
    }
    pub fn is_primitive(&self) -> bool {
        match self {
            ActDataType::Primitive(_) => true,
            _ => false,
        }
    }
    pub fn is_record(&self) -> bool {
        match self {
            ActDataType::Record(_) => true,
            _ => false,
        }
    }
    pub fn is_tuple(&self) -> bool {
        match self {
            ActDataType::Tuple(_) => true,
            _ => false,
        }
    }
    pub fn is_type_ref(&self) -> bool {
        match self {
            ActDataType::TypeRef(_) => true,
            _ => false,
        }
    }
    pub fn is_variant(&self) -> bool {
        match self {
            ActDataType::Variant(_) => true,
            _ => false,
        }
    }
    pub fn needs_definition(&self) -> bool {
        match self {
            ActDataType::Primitive(_) => false,
            ActDataType::TypeRef(_) => false,
            ActDataType::Array(_) => false,
            ActDataType::Option(_) => false,
            ActDataType::Record(act_record) => act_record.act_type.is_literal(),
            ActDataType::Variant(act_variant) => act_variant.act_type.is_literal(),
            ActDataType::Func(act_func) => act_func.act_type.is_literal(),
            ActDataType::Tuple(act_tuple) => act_tuple.act_type.is_literal(),
        }
    }

    pub fn as_type_alias(&self) -> Option<ActDataType> {
        match self {
            ActDataType::Primitive(_) => None,
            ActDataType::Option(_) => None,
            ActDataType::TypeRef(_) => None,
            ActDataType::Array(_) => None,
            ActDataType::Record(record) => Some(ActDataType::Record(record.as_type_alias())),
            ActDataType::Variant(variant) => Some(ActDataType::Variant(variant.as_type_alias())),
            ActDataType::Func(func) => Some(ActDataType::Func(func.as_type_alias())),
            ActDataType::Tuple(tuple) => Some(ActDataType::Tuple(tuple.as_type_alias())),
        }
    }

    pub fn needs_to_be_boxed(&self) -> bool {
        true
    }

    pub fn get_members(&self) -> Vec<ActDataType> {
        match self {
            ActDataType::Record(act_record) => act_record.get_members(),
            ActDataType::Variant(act_variant) => act_variant.get_members(),
            ActDataType::Func(act_func) => act_func.get_members(),
            ActDataType::Primitive(_) => vec![],
            ActDataType::TypeRef(_) => vec![],
            ActDataType::Array(act_array) => act_array.get_members(),
            ActDataType::Tuple(act_tuple) => act_tuple.get_members(),
            ActDataType::Option(act_option) => act_option.get_members(),
        }
    }

    pub fn collect_inline_types(&self) -> Vec<ActDataType> {
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

impl ToTokenStream<&Vec<String>> for ActDataType {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        match self {
            ActDataType::Record(act_record) => act_record.to_token_stream(keyword_list),
            ActDataType::Variant(act_variant) => act_variant.to_token_stream(keyword_list),
            ActDataType::Func(act_func) => act_func.to_token_stream(keyword_list),
            ActDataType::Tuple(act_tuple) => act_tuple.to_token_stream(keyword_list),
            ActDataType::Primitive(act_primitive) => act_primitive.to_token_stream(keyword_list),
            ActDataType::TypeRef(act_type_ref) => act_type_ref.to_token_stream(keyword_list),
            ActDataType::Option(act_option) => act_option.to_token_stream(keyword_list),
            ActDataType::Array(act_array) => act_array.to_token_stream(keyword_list),
        }
    }
}

pub fn build_inline_type_acts(type_aliases: &Vec<ActDataType>) -> Vec<ActDataType> {
    type_aliases.iter().fold(vec![], |acc, type_alias| {
        vec![acc, type_alias.collect_inline_types()].concat()
    })
}

pub fn deduplicate(
    act_data_type_nodes: Vec<ActDataType>,
    keyword_list: &Vec<String>,
) -> Vec<ActDataType> {
    let map: HashMap<String, ActDataType> =
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
