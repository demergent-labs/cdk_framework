use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::{traits::ToTypeAnnotation, DataType};
use crate::{
    act::{proclamation::Proclaim, Declaration, TypeAnnotation},
    traits::ToIdent,
};

#[derive(Clone, Debug)]
pub struct Func {
    pub name: Option<String>,
    pub params: Vec<DataType>,
    pub return_type: Box<DataType>,
    pub mode: Mode,
    pub to_vm_value: TokenStream,
    pub list_to_vm_value: TokenStream,
    pub from_vm_value: TokenStream,
    pub list_from_vm_value: TokenStream,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Query,
    Update,
    Oneway,
}

impl Func {
    fn get_name(&self, parental_prefix: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => format!("{}Func", parental_prefix),
        }
    }
}

impl<C> ToTypeAnnotation<C> for Func {
    fn to_type_annotation(&self, _: &C, parental_prefix: String) -> TypeAnnotation {
        self.get_name(parental_prefix)
            .to_identifier()
            .to_token_stream()
    }
}

impl Proclaim<Vec<String>> for Func {
    fn create_declaration(
        &self,
        keyword_list: &Vec<String>,
        parental_prefix: String,
    ) -> Option<Declaration> {
        Some(self.generate_func_struct_and_impls(
            keyword_list,
            self.create_identifier(parental_prefix).unwrap(),
        ))
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        Some(self.get_name(parental_prefix))
    }

    fn collect_inline_declarations(&self, _: &Vec<String>, _: String) -> Vec<Declaration> {
        // My assumption here is that when we get to rust none of the children
        // that were in the func will need to be defined unless they are used
        // somewhere else, and if that's the case then we will pick it up there.
        vec![]
    }
}

pub fn generate_func_arg_token() -> TokenStream {
    quote! {
        // TODO I think it's debatable whether or not we even need ArgToken
        /// A marker type to match unconstrained callback arguments
        #[derive(Debug, Clone, Copy, PartialEq, candid::Deserialize)]
        pub struct ArgToken;

        impl candid::CandidType for ArgToken {
            fn _ty() -> candid::types::Type {
                candid::types::Type::Empty
            }

            fn idl_serialize<S: candid::types::Serializer>(&self, _serializer: S) -> Result<(), S::Error> {
                // We cannot implement serialize, since our type must be \`Empty\` in order to accept anything.
                // Attempting to serialize this type is always an error and should be regarded as a compile time error.
                unimplemented!("Token is not serializable")
            }
        }
    }
}

impl Func {
    pub fn generate_func_struct_and_impls(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> TokenStream {
        let type_alias_name = name.to_identifier();
        let func_mode = match self.mode {
            Mode::Query => quote! {candid::parser::types::FuncMode::Query },
            Mode::Oneway => quote! {candid::parser::types::FuncMode::Oneway },
            Mode::Update => quote! {},
        };
        let param_type_strings: Vec<String> = self
            .params
            .iter()
            .map(|param| {
                param
                    .to_type_annotation(keyword_list, name.clone())
                    .to_string()
            })
            .collect();
        let func_param_types: Vec<TokenStream> = param_type_strings
            .iter()
            .map(|rust_type| {
                let modified_rust_type = if rust_type.starts_with("Vec") {
                    rust_type
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()
                        .replacen("Vec<", "Vec::<", 1)
                } else if rust_type.starts_with("Option") {
                    rust_type
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>()
                        .replacen("Option<", "Option::<", 1)
                } else {
                    rust_type.clone()
                };

                let modified_rust_type_token_stream: TokenStream = modified_rust_type
                    .parse()
                    // Note: This should be impossible to hit. Anything that isn't
                    // parsable should be caught when going from TS to JS.
                    .expect(&format!(
                        "Unable to parse parameter type {} in Func {}",
                        modified_rust_type, type_alias_name
                    ));

                if rust_type == "(())" {
                    quote! { candid::types::Type::Null }
                } else {
                    quote! { #modified_rust_type_token_stream::_ty() }
                }
            })
            .collect();
        let return_type_string = self
            .return_type
            .to_type_annotation(keyword_list, name)
            .to_string();
        let func_return_type = if return_type_string == "()" || return_type_string == "" {
            quote! {}
        } else if return_type_string == "(())" {
            quote! { candid::types::Type::Null}
        } else {
            let return_type_token_stream: TokenStream = return_type_string
                .parse()
                // Note: This should be impossible to hit. Anything that isn't
                // parsable should be caught when going from TS to JS.
                .expect(&format!(
                    "Unable to parse return type {} in Func {}",
                    return_type_string, type_alias_name
                ));
            quote! { #return_type_token_stream::_ty()}
        };

        let func_to_vm_value = &self.to_vm_value;
        let func_list_to_vm_value = &self.list_to_vm_value;
        let func_from_vm_value = &self.from_vm_value;
        let func_list_from_vm_value = &self.list_from_vm_value;

        quote! {
            #[derive(Debug, Clone)]
            struct #type_alias_name<ArgToken = self::ArgToken>(
                pub candid::Func,
                pub std::marker::PhantomData<ArgToken>,
            );

            #func_to_vm_value
            #func_list_to_vm_value
            #func_from_vm_value
            #func_list_from_vm_value

            impl candid::CandidType for #type_alias_name {
                fn _ty() -> candid::types::Type {
                    candid::types::Type::Func(candid::types::Function {
                        modes: vec![#func_mode],
                        args: vec![#(#func_param_types),*],
                        rets: vec![#func_return_type]
                    })
                }

                fn idl_serialize<S: candid::types::Serializer>(&self, serializer: S) -> Result<(), S::Error> {
                    self.0.idl_serialize(serializer)
                }
            }

            impl<'de> candid::Deserialize<'de> for #type_alias_name {
                fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    candid::Func::deserialize(deserializer).map(Self::from)
                }
            }

            impl From<candid::Func> for #type_alias_name {
                fn from(f: candid::Func) -> Self {
                    Self(f, std::marker::PhantomData)
                }
            }

            impl From<#type_alias_name> for candid::Func {
                fn from(c: #type_alias_name) -> Self {
                    c.0
                }
            }

            impl std::ops::Deref for #type_alias_name {
                type Target = candid::Func;
                fn deref(&self) -> &candid::Func {
                    &self.0
                }
            }

            impl std::ops::DerefMut for #type_alias_name {
                fn deref_mut(&mut self) -> &mut candid::Func {
                    &mut self.0
                }
            }
        }
    }
}
