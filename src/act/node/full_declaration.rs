use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::{
    act::node::canister_method::{HasName, HasParams, HasReturnValue},
    ToDeclarationTokenStream,
};

use super::canister_method::QueryMethod;

#[derive(Clone)]
pub struct Declaration {
    pub identifier: String,
    pub declaration: Option<TokenStream>,
    pub children: Box<HashMap<String, Declaration>>,
}

pub trait ToFullDeclaration<C> {
    fn create_full_declaration(&self, context: &C, parental_prefix: String) -> Declaration {
        Declaration {
            identifier: self.create_identifier(parental_prefix.clone()),
            declaration: self.create_declaration(context, parental_prefix.clone()),
            children: Box::new(self.create_child_declarations(&context, parental_prefix.clone())),
        }
    }
    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<TokenStream>;
    fn create_identifier(&self, parental_prefix: String) -> String;
    fn create_child_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, Declaration>;
}

impl<C, T> ToFullDeclaration<C> for Vec<T>
where
    C: Clone,
    T: ToFullDeclaration<C>,
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

    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<TokenStream> {
        let result_list: Vec<_> = self
            .iter()
            .map(|item| item.create_declaration(context, parental_prefix.clone()))
            .collect();
        Some(quote!(#(#result_list)*))
    }

    fn create_identifier(&self, parental_prefix: String) -> String {
        format!("{}ListOfDeclarations", parental_prefix)
    }
}

impl ToFullDeclaration<Vec<String>> for QueryMethod
// where
//     T: HasParams,
//     T: HasReturnValue,
//     T: HasName,
//     T: ToDeclarationTokenStream<C>,
{
    /// When you create a child declaration what are we trying to accomplish?
    /// We want to flatten it and deduplicate it.
    /// 1) Get full declaration of child
    /// 2) Get self declaration of child
    /// 3) Get grandchildren full declarations
    /// 4) Flatten 2 and 3 into one map
    /// 5) Repeat for all of the children, flattening that into one map.
    /// For the case of the query method, we are going to get the params and return type and do the thing. for each of those
    ///
    fn create_child_declarations(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> HashMap<String, Declaration> {
        let mut result = self.get_param_types().iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (index, param_type)| {
                let param_prefix =
                    format!("{}{}ParamNum{}", parental_prefix, self.get_name(), index);
                let declaration = param_type.create_full_declaration(context, param_prefix);
                acc.extend(declaration.children.clone().into_iter());
                acc.insert(declaration.identifier.clone(), declaration);
                acc
            },
        );
        let return_prefix = format!("{}{}ReturnType", parental_prefix, self.get_name());
        let declaration = self
            .get_return_type()
            .create_full_declaration(&context, return_prefix.clone());
        result.insert(return_prefix, declaration);
        result
    }

    fn create_declaration(
        &self,
        context: &Vec<String>,
        parental_prefix: String,
    ) -> Option<TokenStream> {
        Some(self.to_declaration(
            &context.clone(),
            format!("{}{}", parental_prefix, self.get_name()),
        ))
    }

    fn create_identifier(&self, _: String) -> String {
        self.get_name()
    }
}
