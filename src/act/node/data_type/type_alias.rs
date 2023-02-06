use super::{
    traits::{HasMembers, ToTypeAnnotation},
    DataType,
};
use crate::{act::node::full_declaration::ToDeclaration, traits::ToIdent, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub name: String,
    pub aliased_type: Box<DataType>,
}

impl HasMembers for TypeAlias {
    fn get_members(&self) -> Vec<DataType> {
        vec![*self.aliased_type.clone()]
    }

    fn create_member_prefix(&self, index: usize, parental_prefix: String) -> String {
        todo!("I don't think we will be using this. Is HasMembers also not good for this")
    }
}

impl ToTypeAnnotation<Vec<String>> for TypeAlias {
    fn to_type_annotation(&self, _: &Vec<String>, _: String) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<Vec<String>> for TypeAlias {
    fn to_token_stream(&self, context: &Vec<String>) -> TokenStream {
        self.to_type_annotation(context, "".to_string())
    }
}

impl ToDeclaration<Vec<String>> for TypeAlias {
    fn create_code(&self, context: &Vec<String>, parental_prefix: String) -> Option<TokenStream> {
        let name = self.name.to_identifier();
        let alias = self
            .aliased_type
            .to_type_annotation(context, parental_prefix);
        Some(quote!(type #name = #alias;))
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        Some(self.name.clone())
    }

    fn create_child_declarations(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> std::collections::HashMap<String, crate::act::node::full_declaration::Declaration> {
        self.aliased_type
            .create_child_declarations(context, parental_prefix)
    }
}
