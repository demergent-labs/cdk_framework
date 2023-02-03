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
pub mod type_alias;
pub mod type_ref;
pub mod variant;

use crate::ToTokenStream;
use proc_macro2::TokenStream;
use std::collections::HashMap;

pub use array::Array;
pub use func::Func;
pub use option::Option;
pub use primitive::Primitive;
pub use record::Record;
pub use tuple::Tuple;
pub use type_alias::TypeAlias;
pub use type_ref::TypeRef;
pub use variant::Variant;

use self::traits::HasMembers;

use super::full_declaration::{Declaration, ToFullDeclaration};

#[derive(Clone, Debug)]
pub enum DataType {
    Array(Array),
    Func(Func),
    Option(self::Option),
    Primitive(Primitive),
    Record(Record),
    Tuple(Tuple),
    TypeAlias(TypeAlias),
    TypeRef(TypeRef),
    Variant(Variant),
}

impl DataType {
    pub fn as_array(&self) -> core::option::Option<&Array> {
        match self {
            DataType::Array(array) => Some(&array),
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

    pub fn as_type_ref(&self) -> core::option::Option<&TypeRef> {
        match self {
            DataType::TypeRef(type_ref) => Some(&type_ref),
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
            DataType::TypeAlias(_) => true,
            DataType::Record(_) => true,
            DataType::Variant(_) => true,
            DataType::Func(_) => true,
            DataType::Tuple(_) => true,
            DataType::Primitive(_) => false,
            DataType::TypeRef(_) => false,
            DataType::Array(_) => false,
            DataType::Option(_) => false,
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
            DataType::TypeAlias(type_alias) => type_alias.get_members(),
            DataType::Array(act_array) => act_array.get_members(),
            DataType::Tuple(act_tuple) => act_tuple.get_members(),
            DataType::Option(act_option) => act_option.get_members(),
        }
    }

    pub fn collect_inline_types(&self) -> Vec<DataType> {
        let act_data_type = match self.needs_definition() {
            true => vec![self.clone()],
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
            DataType::TypeAlias(act_type_alias) => act_type_alias.to_token_stream(keyword_list),
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

pub fn new_deduplicate<C, T>(nodes: &Vec<T>, context: C) -> Vec<T>
where
    C: Clone,
    T: ToTokenStream<C>,
    T: Clone,
{
    let map: HashMap<String, T> = nodes.iter().fold(HashMap::new(), |mut acc, node| {
        match acc.get(&node.to_token_stream(context.clone()).to_string()) {
            Some(_) => acc,
            None => {
                acc.insert(
                    node.to_token_stream(context.clone()).to_string(),
                    node.clone(),
                );
                acc
            }
        }
    });
    map.values().cloned().collect()
}

impl ToFullDeclaration<Vec<String>> for DataType {
    fn create_child_declarations(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, Declaration> {
        // TODO
        HashMap::new()
    }

    fn create_declaration(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> std::option::Option<TokenStream> {
        let prefix = format!("DataType{}", parental_prefix);
        match self {
            DataType::Array(_) => todo!(),
            DataType::Func(_) => todo!(),
            DataType::Option(_) => todo!(),
            DataType::Primitive(_) => todo!(),
            DataType::Record(record) => record.create_declaration(context, prefix),
            DataType::Tuple(_) => todo!(),
            DataType::TypeAlias(type_alias) => type_alias.create_declaration(context, prefix),
            DataType::TypeRef(type_ref) => type_ref.create_declaration(context, prefix),
            DataType::Variant(_) => todo!(),
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> String {
        match self {
            DataType::Array(_) => "Array".to_string(),
            DataType::Func(func) => match &func.name {
                Some(name) => name.clone(),
                None => format!("{}Func", parental_prefix),
            },
            DataType::Option(_) => "Option".to_string(),
            DataType::Primitive(_) => todo!(),
            DataType::Record(record) => record.create_identifier(parental_prefix),
            DataType::Tuple(_) => todo!(),
            DataType::TypeAlias(_) => todo!(),
            DataType::TypeRef(_) => "TypeRef".to_string(),
            DataType::Variant(_) => todo!(),
        }
    }
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
