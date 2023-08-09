use quote::{quote, ToTokens};

use crate::{
    act::{node::Context, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{HasTypeRefs, ToIdent, ToTokenStream},
};

use super::TypeArg;

#[derive(Clone, Debug)]
pub struct TypeRef {
    pub name: String,
    pub type_arguments: Vec<TypeArg>,
}

impl ToTypeAnnotation<Context> for TypeRef {
    fn to_type_annotation(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> TypeAnnotation {
        if let Some(module_name_string) = module_name {
            let module_name_ident = module_name_string.to_ident();

            // TODO use the keyword list to make the identifier rust safe
            let name = self.name.to_ident().to_token_stream();
            let type_arguments_token_stream =
                self.type_arguments.to_token_stream(context, &inline_name);

            // TODO this will break with nested modules
            quote!(crate::#module_name_ident::#name #type_arguments_token_stream)
        } else {
            // TODO use the keyword list to make the identifier rust safe
            let name = self.name.to_ident().to_token_stream();
            let type_arguments_token_stream =
                self.type_arguments.to_token_stream(context, &inline_name);

            quote!(#name #type_arguments_token_stream)
        }
    }
}

impl Declare<Context> for TypeRef {
    fn to_declaration(&self, _: &Context, _: String, _: &Option<String>) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.type_arguments
            .iter()
            .enumerate()
            .fold(vec![], |acc, (index, item)| {
                vec![
                    acc,
                    item.flatten(
                        context,
                        item.get_inline_name(&inline_name, index),
                        module_name,
                    ),
                ]
                .concat()
            })
    }
}

impl HasTypeRefs for TypeRef {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        vec![
            vec![self.clone()],
            self.type_arguments
                .iter()
                .flat_map(|ta| ta.get_type_refs())
                .collect(),
        ]
        .concat()
    }
}
