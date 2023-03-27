use quote::{quote, ToTokens};

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
}

impl ToTypeAnnotation<Context> for TypeRef {
    fn to_type_annotation(&self, _: &Context, _: String) -> TypeAnnotation {
        // TODO use the keyword list to make the identifier rust safe
        let ident = self.name.to_ident().to_token_stream();
        quote!(#ident)
    }
}

impl Declare<Context> for TypeRef {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(&self, _: &Context, _: String) -> Vec<Declaration> {
        vec![]
    }
}
