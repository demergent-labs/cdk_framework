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
pub struct Opt {
    pub enclosed_type: Box<CandidType>,
}

impl ToTypeAnnotation<Context> for Opt {
    fn to_type_annotation(
        &self,
        context: &Context,
        inline_name: String,
        module_name_option: &Option<String>,
    ) -> TypeAnnotation {
        let enclosed_type_annotation =
            self.enclosed_type
                .to_type_annotation(context, inline_name, module_name_option);
        quote!(Option<#enclosed_type_annotation>)
    }
}

impl Declare<Context> for Opt {
    fn to_declaration(&self, _: &Context, _: String, _: &Option<String>) -> Option<Declaration> {
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

impl HasTypeRefs for Opt {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.enclosed_type.get_type_refs()
    }
}
