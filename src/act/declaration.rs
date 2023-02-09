use proc_macro2::TokenStream;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Declaration {
    pub identifier: Option<String>,
    pub code: Option<TokenStream>,
    pub children: HashMap<String, TokenStream>,
}

pub trait ToDeclaration<C> {
    fn create_declaration(&self, context: &C, parental_prefix: String) -> Declaration {
        Declaration {
            identifier: self.create_identifier(parental_prefix.clone()),
            code: self.create_code(context, parental_prefix.clone()),
            children: self.create_child_declarations(&context, parental_prefix.clone()),
        }
    }
    fn create_code(&self, context: &C, parental_prefix: String) -> Option<TokenStream>;
    fn create_identifier(&self, parental_prefix: String) -> Option<String>;
    fn create_child_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream>;
}

impl<C, T> ToDeclaration<C> for Vec<T>
where
    C: Clone,
    T: ToDeclaration<C>,
{
    fn create_child_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.iter().fold(HashMap::new(), |acc, declaration| {
            let decl = declaration.create_declaration(context, parental_prefix.clone());
            super::add_declaration_to_map(decl, acc)
        })
    }

    fn create_code(&self, _: &C, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }
}
