use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

// TODO this is weird and needs to be worked out. Remember this should not be a
// tree. It should be a flat, here is a thing and here are all of the children
// and grandchildren flattened into a single collection
#[derive(Clone)]
pub struct Declaration {
    pub identifier: Option<String>,
    pub code: Option<TokenStream>,
    pub children: Box<HashMap<String, Declaration>>, // For example this might be a hashmap of <String, code>
}

pub trait ToDeclaration<C> {
    fn create_declaration(&self, context: &C, parental_prefix: String) -> Declaration {
        Declaration {
            identifier: self.create_identifier(parental_prefix.clone()),
            code: self.create_code(context, parental_prefix.clone()),
            children: Box::new(self.create_child_declarations(&context, parental_prefix.clone())),
        }
    }
    fn create_code(&self, context: &C, parental_prefix: String) -> Option<TokenStream>;
    fn create_identifier(&self, parental_prefix: String) -> Option<String>;
    fn create_child_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, Declaration>;
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
    ) -> HashMap<String, Declaration> {
        self.iter().fold(HashMap::new(), |mut acc, declaration| {
            let children = declaration.create_child_declarations(context, parental_prefix.clone());
            acc.extend(children);
            acc
        })
    }

    fn create_code(&self, context: &C, parental_prefix: String) -> Option<TokenStream> {
        let result_list: Vec<_> = self
            .iter()
            .map(|item| item.create_code(context, parental_prefix.clone()))
            .collect();
        Some(quote!(#(#result_list)*))
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        Some(format!("{}ListOfDeclarations", parental_prefix))
    }
}
