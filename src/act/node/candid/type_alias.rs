use quote::{quote, ToTokens};

use crate::{
    act::{
        node::{CandidType, Context},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasTypeRefs, ToIdent},
};

use super::{type_param::TypeParams, TypeRef};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<CandidType>,
    pub type_params: TypeParams,
}

impl ToTypeAnnotation<Context> for TypeAlias {
    fn to_type_annotation(&self, _: &Context, _: String) -> TypeAnnotation {
        self.name.to_ident().to_token_stream()
    }
}

impl Declare<Context> for TypeAlias {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let name = self.name.to_ident();
        let alias = self
            .aliased_type
            .to_type_annotation(context, self.name.clone());
        let type_params_token_stream = self.type_params.get_type_params_token_stream();
        let where_clause_token_stream = self.type_params.get_where_clause_token_stream();

        Some(quote!(type #name #type_params_token_stream #where_clause_token_stream = #alias;))
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.aliased_type.flatten(context, self.name.clone())
    }
}

impl HasTypeRefs for TypeAlias {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.aliased_type
            .as_type_ref()
            .into_iter()
            .map(|type_ref| type_ref)
            .collect()
    }
}
