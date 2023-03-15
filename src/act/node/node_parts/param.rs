use proc_macro2::TokenStream;

use crate::{
    act::{node::CandidType, Declaration, Declare, ToTypeAnnotation, TypeAnnotation},
    traits::{HasInlineName, ToIdent},
};

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub candid_type: CandidType,
}

impl Param {
    pub fn get_prefixed_name(&self) -> String {
        format!("_cdk_user_defined_{name}", name = self.name)
    }

    pub fn to_token_stream(
        &self,
        keyword_list: &Vec<String>,
        function_name: String,
    ) -> TokenStream {
        let name = self.get_prefixed_name().to_ident();
        let function_name = self.to_type_annotation(keyword_list, function_name);
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

impl ToTypeAnnotation<Vec<String>> for Param {
    fn to_type_annotation(
        &self,
        keyword_list: &Vec<String>,
        function_name: String,
    ) -> TypeAnnotation {
        self.candid_type
            .to_type_annotation(keyword_list, self.get_inline_name(&function_name))
    }
}

impl Declare<Vec<String>> for Param {
    fn to_declaration(&self, _: &Vec<String>, _: String) -> Option<Declaration> {
        None
    }

    fn collect_inline_declarations(
        &self,
        context: &Vec<String>,
        function_name: String,
    ) -> Vec<Declaration> {
        self.candid_type
            .flatten(context, self.get_inline_name(&function_name))
    }
}
