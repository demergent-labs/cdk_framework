use proc_macro2::TokenStream;

use crate::{
    act::node::{candid::type_annotation::ToTypeAnnotation, CandidType},
    traits::ToIdent,
};

use super::{candid::type_annotation::TypeAnnotation, declaration::Declare, Declaration};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub candid_type: CandidType,
}

impl Param {
    pub fn prefixed_name(&self) -> String {
        format!("_cdk_user_defined_{}", self.name)
    }

    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        function_prefix: String,
    ) -> TokenStream {
        let name = self.prefixed_name().to_ident();
        let type_annotation = self.to_type_annotation(keyword_list, function_prefix);
        quote::quote! {
            #name: #type_annotation
        }
    }

    fn get_name(&self, function_prefix: String) -> String {
        format!("{}{}", function_prefix, self.prefixed_name())
    }
}

impl ToTypeAnnotation<Vec<String>> for Param {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        function_prefix: String,
    ) -> TypeAnnotation {
        self.candid_type
            .to_type_annotation(keyword_list, self.get_name(function_prefix))
    }
}

impl Declare<Vec<String>> for Param {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Vec<String>,
        function_prefix: String,
    ) -> Vec<Declaration> {
        self.candid_type
            .flatten(context, self.get_name(function_prefix))
    }
}
