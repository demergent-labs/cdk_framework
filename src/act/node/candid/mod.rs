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
    fn to_type_annotation(&self, context: &Context, inline_name: String) -> TypeAnnotation {
        let f =
            |m: &dyn ToTypeAnnotation<Context>| m.to_type_annotation(context, inline_name.clone());
        match self {
            CandidType::Array(array) => f(array),
            CandidType::Func(func) => f(func),
            CandidType::Opt(opt) => f(opt),
            CandidType::Record(record) => f(record),
            CandidType::Service(service) => f(service),
            CandidType::Tuple(tuple) => f(tuple),
            CandidType::TypeAlias(type_alias) => f(type_alias),
            CandidType::TypeParam(type_param) => f(type_param),
            CandidType::TypeRef(type_ref) => f(type_ref),
            CandidType::Variant(variant) => f(variant),
            CandidType::Primitive(primitive) => {
                primitive.to_type_annotation(&context.keyword_list, inline_name)
            }
        }
    }
}

impl Declare<Context> for CandidType {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        let f = |m: &dyn Declare<Context>| m.to_declaration(context, inline_name.clone());
        match self {
            CandidType::Array(array) => f(array),
            CandidType::Func(func) => f(func),
            CandidType::Opt(opt) => f(opt),
            CandidType::Record(record) => f(record),
            CandidType::Service(service) => f(service),
            CandidType::Tuple(tuple) => f(tuple),
            CandidType::TypeAlias(type_alias) => f(type_alias),
            CandidType::TypeParam(type_param) => f(type_param),
            CandidType::TypeRef(type_ref) => f(type_ref),
            CandidType::Variant(variant) => f(variant),
            CandidType::Primitive(primitive) => {
                primitive.to_declaration(&context.keyword_list, inline_name)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        let f =
            |m: &dyn Declare<Context>| m.collect_inline_declarations(context, inline_name.clone());
        match self {
            CandidType::Array(array) => f(array),
            CandidType::Func(func) => f(func),
            CandidType::Opt(opt) => f(opt),
            CandidType::Record(record) => f(record),
            CandidType::Service(service) => f(service),
            CandidType::Tuple(tuple) => f(tuple),
            CandidType::TypeAlias(type_alias) => f(type_alias),
            CandidType::TypeParam(type_param) => f(type_param),
            CandidType::TypeRef(type_ref) => f(type_ref),
            CandidType::Variant(variant) => f(variant),
            CandidType::Primitive(primitive) => {
                primitive.collect_inline_declarations(&context.keyword_list, inline_name)
            }
        }
    }
}

impl HasTypeRefs for CandidType {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        let f = |m: &dyn HasTypeRefs| m.get_type_refs();
        match self {
            CandidType::Array(array) => f(array),
            CandidType::Func(func) => f(func),
            CandidType::Opt(opt) => f(opt),
            CandidType::Record(record) => f(record),
            CandidType::Service(service) => f(service),
            CandidType::Tuple(tuple) => f(tuple),
            CandidType::TypeAlias(type_alias) => f(type_alias),
            CandidType::Variant(variant) => f(variant),
            CandidType::Primitive(_) => vec![],
            CandidType::TypeParam(_) => vec![],
            CandidType::TypeRef(type_ref) => vec![type_ref.clone()],
        }
    }
}
