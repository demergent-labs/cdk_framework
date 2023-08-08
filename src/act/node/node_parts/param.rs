use proc_macro2::TokenStream;

use crate::{
    act::{
        node::{CandidType, Context},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasInlineName, ToIdent, WithUserDefinedPrefix},
};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub candid_type: CandidType,
}

impl Param {
    pub fn get_prefixed_name(&self) -> String {
        self.name.with_user_defined_prefix()
    }

    pub fn to_token_stream(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> TokenStream {
        let name = self.get_prefixed_name().to_ident();
        let function_name = self.to_type_annotation(context, function_name, module_name);
        quote::quote! {
            #name: #function_name
        }
    }
}

impl HasInlineName for Param {
    fn get_inline_name(&self, function_name: &String) -> String {
        format!("{function_name}_{param_name}", param_name = self.name)
    }
}

impl ToTypeAnnotation<Context> for Param {
    fn to_type_annotation(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> TypeAnnotation {
        self.candid_type.to_type_annotation(
            context,
            self.get_inline_name(&function_name),
            module_name,
        )
    }
}

impl Declare<Context> for Param {
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
        function_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.candid_type
            .flatten(context, self.get_inline_name(&function_name), module_name)
    }
}
