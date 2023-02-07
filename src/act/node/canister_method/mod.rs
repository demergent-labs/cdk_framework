pub mod fn_param;
pub mod heartbeat_method;
pub mod init_method;
pub mod inspect_message_method;
pub mod post_upgrade_method;
pub mod pre_upgrade_method;
pub mod query_method;
pub mod update_method;

use std::collections::HashMap;

pub use fn_param::FnParam;
pub use heartbeat_method::HeartbeatMethod;
pub use init_method::InitMethod;
pub use inspect_message_method::InspectMessageMethod;
pub use post_upgrade_method::PostUpgradeMethod;
pub use pre_upgrade_method::PreUpgradeMethod;
use proc_macro2::TokenStream;
pub use query_method::QueryMethod;
pub use update_method::UpdateMethod;

use quote::quote;

use super::{data_type::traits::ToTypeAnnotation, declaration::ToDeclaration, DataType};

#[derive(Clone)]
pub enum CanisterMethod {
    Update(UpdateMethod),
    Query(QueryMethod),
    Init(InitMethod),
    PreUpgrade(PreUpgradeMethod),
    PostUpgrade(PostUpgradeMethod),
    InspectMessage(InspectMessageMethod),
    Heartbeat(HeartbeatMethod),
}

pub trait GetAllTypes {
    fn get_all_types(&self) -> Vec<DataType>;
}

impl<T> GetAllTypes for Vec<T>
where
    T: GetAllTypes,
{
    fn get_all_types(&self) -> Vec<DataType> {
        self.iter().fold(vec![], |acc, canister_method| {
            let inline_types = canister_method.get_all_types();
            vec![acc, inline_types].concat()
        })
    }
}

impl<T> GetAllTypes for T
where
    T: HasParams,
    T: HasReturnValue,
{
    fn get_all_types(&self) -> Vec<DataType> {
        vec![self.get_param_types(), vec![self.get_return_type()]].concat()
    }
}

pub trait HasReturnValue {
    fn get_return_type(&self) -> DataType;
    fn create_return_type_prefix(&self) -> String;
    fn create_return_type_declarations(
        &self,
        keyword_list: &Vec<String>,
    ) -> HashMap<String, TokenStream> {
        let mut result = HashMap::new();
        let declaration = self
            .get_return_type()
            .create_declaration(&keyword_list, self.create_return_type_prefix());

        if let Some(identifier) = declaration.identifier {
            if let Some(code) = declaration.code {
                result.insert(identifier, code);
            }
        }
        result.extend(declaration.children);

        result
    }
    fn create_return_type_annotation(&self, keyword_list: &Vec<String>) -> TokenStream {
        self.get_return_type()
            .to_type_annotation(keyword_list, self.create_return_type_prefix())
    }
}

pub trait HasParams {
    fn get_param_types(&self) -> Vec<DataType> {
        self.get_params()
            .iter()
            .map(|param| param.data_type.clone())
            .collect()
    }
    fn get_params(&self) -> Vec<FnParam>;
    fn create_param_prefix(&self, param_index: usize) -> String;
    fn create_parameter_list_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .enumerate()
            .map(|(index, param)| {
                param.to_token_stream(keyword_list, self.create_param_prefix(index))
            })
            .collect();
        quote!(#(#params),*)
    }
    fn create_param_type_annotation(
        &self,
        param_index: usize,
        keyword_list: &Vec<String>,
    ) -> Option<TokenStream> {
        match self.get_params().get(param_index) {
            Some(param) => Some(
                param
                    .data_type
                    .to_type_annotation(keyword_list, self.create_param_prefix(param_index)),
            ),
            None => None,
        }
    }
    fn create_param_declarations(
        &self,
        keyword_list: &Vec<String>,
    ) -> HashMap<String, TokenStream> {
        self.get_param_types().iter().enumerate().fold(
            HashMap::new(),
            |mut acc, (index, param_type)| {
                let declaration =
                    param_type.create_declaration(keyword_list, self.create_param_prefix(index));
                if let Some(identifier) = &declaration.identifier {
                    if let Some(code) = declaration.code {
                        acc.insert(identifier.clone(), code.clone());
                    }
                }
                acc.extend(declaration.children.clone().into_iter());
                acc
            },
        )
    }
}

pub trait HasName {
    fn get_name(&self) -> String;
}
