use crate::act::{Declaration, Declare, ToTypeAnnotation, TypeAnnotation};

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

impl CandidType {
    pub fn as_type_ref(&self) -> Option<TypeRef> {
        match self {
            CandidType::TypeRef(type_ref) => Some(type_ref.clone()),
            _ => None,
        }
    }
}

impl ToTypeAnnotation<Context> for CandidType {
    fn to_type_annotation(&self, context: &Context, inline_name: String) -> TypeAnnotation {
        match self {
            CandidType::Array(array) => array.to_type_annotation(context, inline_name),
            CandidType::Func(func) => func.to_type_annotation(context, inline_name),
            CandidType::Opt(opt) => opt.to_type_annotation(context, inline_name),
            CandidType::Primitive(primitive) => primitive.to_type_annotation(context, inline_name),
            CandidType::Record(record) => record.to_type_annotation(context, inline_name),
            CandidType::Service(service) => service.to_type_annotation(context, inline_name),
            CandidType::Tuple(tuple) => tuple.to_type_annotation(context, inline_name),
            CandidType::TypeAlias(type_alias) => {
                type_alias.to_type_annotation(context, inline_name)
            }
            CandidType::TypeParam(type_param) => {
                type_param.to_type_annotation(context, inline_name)
            }
            CandidType::TypeRef(type_ref) => type_ref.to_type_annotation(context, inline_name),
            CandidType::Variant(variant) => variant.to_type_annotation(context, inline_name),
        }
    }
}

impl Declare<Context> for CandidType {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        match self {
            CandidType::Array(array) => array.to_declaration(context, inline_name),
            CandidType::Func(func) => func.to_declaration(context, inline_name),
            CandidType::Opt(opt) => opt.to_declaration(context, inline_name),
            CandidType::Primitive(primitive) => {
                primitive.to_declaration(&context.keyword_list, inline_name)
            }
            CandidType::Record(record) => record.to_declaration(context, inline_name),
            CandidType::Service(service) => service.to_declaration(context, inline_name),
            CandidType::Tuple(tuple) => tuple.to_declaration(context, inline_name),
            CandidType::TypeAlias(type_alias) => type_alias.to_declaration(context, inline_name),
            CandidType::TypeParam(type_param) => type_param.to_declaration(context, inline_name),
            CandidType::TypeRef(type_ref) => type_ref.to_declaration(context, inline_name),
            CandidType::Variant(variant) => variant.to_declaration(context, inline_name),
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        match self {
            CandidType::Array(array) => array.collect_inline_declarations(context, inline_name),
            CandidType::Func(func) => func.collect_inline_declarations(context, inline_name),
            CandidType::Opt(opt) => opt.collect_inline_declarations(context, inline_name),
            CandidType::Primitive(primitive) => {
                primitive.collect_inline_declarations(&context.keyword_list, inline_name)
            }
            CandidType::Record(record) => record.collect_inline_declarations(context, inline_name),
            CandidType::Service(service) => {
                service.collect_inline_declarations(context, inline_name)
            }
            CandidType::Tuple(tuple) => tuple.collect_inline_declarations(context, inline_name),
            CandidType::TypeAlias(type_alias) => {
                type_alias.collect_inline_declarations(context, inline_name)
            }
            CandidType::TypeParam(type_param) => {
                type_param.collect_inline_declarations(context, inline_name)
            }
            CandidType::TypeRef(type_ref) => {
                type_ref.collect_inline_declarations(context, inline_name)
            }
            CandidType::Variant(variant) => {
                variant.collect_inline_declarations(context, inline_name)
            }
        }
    }
}
