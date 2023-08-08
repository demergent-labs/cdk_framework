use quote::quote;

use crate::{
    act::{
        node::{CandidType, Context},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::HasTypeRefs,
};

use super::TypeRef;

#[derive(Clone, Debug)]
pub struct Array {
    pub enclosed_type: Box<CandidType>,
}

impl ToTypeAnnotation<Context> for Array {
    fn to_type_annotation(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> TypeAnnotation {
        let enclosed_rust_ident =
            self.enclosed_type
                .to_type_annotation(context, inline_name, module_name);
        quote!(Vec<#enclosed_rust_ident>)
    }
}

impl Declare<Context> for Array {
    fn to_declaration(
        &self,
        _: &Context,
        _: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.enclosed_type
            .flatten(context, inline_name, module_name)
    }
}

impl HasTypeRefs for Array {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.enclosed_type.get_type_refs()
    }
}
