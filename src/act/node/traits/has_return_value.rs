use proc_macro2::TokenStream;
use std::collections::HashMap;

use crate::act::{
    self,
    declaration::ToDeclaration,
    node::data_type::{traits::ToTypeAnnotation, DataType},
};

pub trait HasReturnValue {
    fn get_return_type(&self) -> DataType;
    fn create_return_type_prefix(&self) -> String;

    fn create_return_type_declarations(
        &self,
        keyword_list: &Vec<String>,
    ) -> HashMap<String, TokenStream> {
        let declaration = self
            .get_return_type()
            .create_declaration(&keyword_list, self.create_return_type_prefix());

        act::flatten_declaration(declaration, HashMap::new())
    }

    fn create_return_type_annotation(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix())
    }
}
