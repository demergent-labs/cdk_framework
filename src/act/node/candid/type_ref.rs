use quote::{quote, ToTokens};

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{ToIdent, ToTokenStream},
};

use super::TypeArg;

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
    pub type_arguments: Vec<TypeArg>,
}

impl ToTypeAnnotation<Context> for TypeRef {
    fn to_type_annotation(&self, context: &Context, inline_name: String) -> TypeAnnotation {
        // TODO use the keyword list to make the identifier rust safe
        let name = self.name.to_ident().to_token_stream();
        let type_arguments_token_stream =
            self.type_arguments.to_token_stream(context, &inline_name);

        quote!(#name #type_arguments_token_stream)
    }
}

impl Declare<Context> for TypeRef {
    fn to_declaration(&self, _: &Context, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.type_arguments
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, item)| {
                vec![
                    acc,
                    item.0
                        .flatten(context, item.get_inline_name(&inline_name, index)),
                ]
                .concat()
            })
    }
}
