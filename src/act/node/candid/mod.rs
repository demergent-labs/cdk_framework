use crate::{
    act::{Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::HasTypeRefs,
};

pub mod array;
pub mod func;
pub mod opt;
pub mod primitive;
pub mod record;
pub mod service;
pub mod tuple;
pub mod type_alias;
pub mod type_arg;
pub mod type_param;
pub mod type_ref;
pub mod variant;

pub use array::Array;
pub use func::Func;
pub use opt::Opt;
pub use primitive::Primitive;
pub use record::Record;
pub use service::Service;
pub use tuple::Tuple;
pub use type_alias::TypeAlias;
pub use type_arg::TypeArg;
pub use type_param::TypeParam;
pub use type_ref::TypeRef;
pub use variant::Variant;

use super::{AsNode, Context, Node};

#[derive(Clone, Debug)]
pub enum CandidType {
    Array(Array),
    Func(Func),
    Opt(Opt),
    Primitive(Primitive),
    Record(Record),
    Service(Service),
    Tuple(Tuple),
    TypeAlias(TypeAlias),
    TypeParam(TypeParam),
    TypeRef(TypeRef),
    Variant(Variant),
}

impl AsNode for CandidType {
    fn as_node(self) -> Node {
        Node::CandidType(self)
    }
}

impl ToTypeAnnotation<Context> for CandidType {
    fn to_type_annotation(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> TypeAnnotation {
        let to_type_annotation = |m: &dyn ToTypeAnnotation<Context>| {
            m.to_type_annotation(context, inline_name.clone(), module_name)
        };
        match self {
            CandidType::Array(array) => to_type_annotation(array),
            CandidType::Func(func) => to_type_annotation(func),
            CandidType::Opt(opt) => to_type_annotation(opt),
            CandidType::Record(record) => to_type_annotation(record),
            CandidType::Service(service) => to_type_annotation(service),
            CandidType::Tuple(tuple) => to_type_annotation(tuple),
            CandidType::TypeAlias(type_alias) => to_type_annotation(type_alias),
            CandidType::TypeParam(type_param) => to_type_annotation(type_param),
            CandidType::TypeRef(type_ref) => to_type_annotation(type_ref),
            CandidType::Variant(variant) => to_type_annotation(variant),
            CandidType::Primitive(primitive) => {
                primitive.to_type_annotation(&context.keyword_list, inline_name, module_name)
            }
        }
    }
}

impl Declare<Context> for CandidType {
    fn to_declaration(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        let to_declaration =
            |m: &dyn Declare<Context>| m.to_declaration(context, inline_name.clone(), module_name);
        match self {
            CandidType::Array(array) => to_declaration(array),
            CandidType::Func(func) => to_declaration(func),
            CandidType::Opt(opt) => to_declaration(opt),
            CandidType::Record(record) => to_declaration(record),
            CandidType::Service(service) => to_declaration(service),
            CandidType::Tuple(tuple) => to_declaration(tuple),
            CandidType::TypeAlias(type_alias) => to_declaration(type_alias),
            CandidType::TypeParam(type_param) => to_declaration(type_param),
            CandidType::TypeRef(type_ref) => to_declaration(type_ref),
            CandidType::Variant(variant) => to_declaration(variant),
            CandidType::Primitive(primitive) => {
                primitive.to_declaration(&context.keyword_list, inline_name, module_name)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        let collect_inline_declarations = |m: &dyn Declare<Context>| {
            m.collect_inline_declarations(context, inline_name.clone(), module_name)
        };
        match self {
            CandidType::Array(array) => collect_inline_declarations(array),
            CandidType::Func(func) => collect_inline_declarations(func),
            CandidType::Opt(opt) => collect_inline_declarations(opt),
            CandidType::Record(record) => collect_inline_declarations(record),
            CandidType::Service(service) => collect_inline_declarations(service),
            CandidType::Tuple(tuple) => collect_inline_declarations(tuple),
            CandidType::TypeAlias(type_alias) => collect_inline_declarations(type_alias),
            CandidType::TypeParam(type_param) => collect_inline_declarations(type_param),
            CandidType::TypeRef(type_ref) => collect_inline_declarations(type_ref),
            CandidType::Variant(variant) => collect_inline_declarations(variant),
            CandidType::Primitive(primitive) => primitive.collect_inline_declarations(
                &context.keyword_list,
                inline_name,
                module_name,
            ),
        }
    }
}

impl HasTypeRefs for CandidType {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        let get_type_refs = |m: &dyn HasTypeRefs| m.get_type_refs();
        match self {
            CandidType::Array(array) => get_type_refs(array),
            CandidType::Func(func) => get_type_refs(func),
            CandidType::Opt(opt) => get_type_refs(opt),
            CandidType::Record(record) => get_type_refs(record),
            CandidType::Service(service) => get_type_refs(service),
            CandidType::Tuple(tuple) => get_type_refs(tuple),
            CandidType::TypeAlias(type_alias) => get_type_refs(type_alias),
            CandidType::Variant(variant) => get_type_refs(variant),
            CandidType::TypeRef(type_ref) => get_type_refs(type_ref),
            CandidType::Primitive(_) => vec![],
            CandidType::TypeParam(_) => vec![],
        }
    }
}
