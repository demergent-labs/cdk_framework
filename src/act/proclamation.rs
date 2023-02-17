use proc_macro2::TokenStream;

use super::Declaration;

#[derive(Clone)]
pub struct Proclamation {
    pub identifier: Option<String>,
    pub declaration: Option<Declaration>,
    pub inline_declarations: Vec<Declaration>,
}

pub trait Proclaim<C> {
    fn create_proclamation(&self, context: &C, parental_prefix: String) -> Proclamation {
        Proclamation {
            identifier: self.create_identifier(parental_prefix.clone()),
            declaration: self.create_declaration(context, parental_prefix.clone()),
            inline_declarations: self
                .collect_inline_declarations(&context, parental_prefix.clone()),
        }
    }

    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<Declaration>;
    fn create_identifier(&self, parental_prefix: String) -> Option<String>;
    fn collect_inline_declarations(&self, context: &C, parental_prefix: String)
        -> Vec<Declaration>;
}

impl<C, T> Proclaim<C> for Vec<T>
where
    C: Clone,
    T: Proclaim<C>,
{
    fn collect_inline_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> Vec<TokenStream> {
        self.iter().fold(vec![], |acc, proclaimable| {
            let proclamation = proclaimable.create_proclamation(context, parental_prefix.clone());
            vec![acc, super::flatten_proclamation(&proclamation)].concat()
        })
    }

    fn create_declaration(&self, _: &C, _: String) -> Option<TokenStream> {
        None
    }

    fn create_identifier(&self, _: String) -> Option<String> {
        None
    }
}

impl<C, T> Proclaim<C> for Option<T>
where
    T: Proclaim<C>,
{
    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<TokenStream> {
        match self {
            Some(t) => t.create_declaration(context, format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        match self {
            Some(t) => t.create_identifier(format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> Vec<TokenStream> {
        match self {
            Some(t) => {
                t.collect_inline_declarations(context, format!("{}Optional", parental_prefix))
            }
            None => vec![],
        }
    }
}
