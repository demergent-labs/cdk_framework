use proc_macro2::TokenStream;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Proclamation {
    pub identifier: Option<String>,
    pub declaration: Option<TokenStream>,
    pub inline_declarations: HashMap<String, TokenStream>,
}

pub trait Proclaim<C> {
    fn create_proclamation(&self, context: &C, parental_prefix: String) -> Proclamation {
        Proclamation {
            identifier: self.create_identifier(parental_prefix.clone()),
            declaration: self.create_declaration(context, parental_prefix.clone()),
            inline_declarations: self.create_inline_declarations(&context, parental_prefix.clone()),
        }
    }

    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<TokenStream>;
    fn create_identifier(&self, parental_prefix: String) -> Option<String>;
    fn create_inline_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream>;
}

impl<C, T> Proclaim<C> for Vec<T>
where
    C: Clone,
    T: Proclaim<C>,
{
    fn create_inline_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        self.iter().fold(HashMap::new(), |acc, declaration| {
            let decl = declaration.create_proclamation(context, parental_prefix.clone());
            super::flatten_declaration(decl, acc)
        })
    }

    fn create_declaration(&self, _: &C, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }
}
