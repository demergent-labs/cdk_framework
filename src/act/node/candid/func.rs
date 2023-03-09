use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    act::{
        node::{CandidType, Param, ReturnType},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasInlines, IsCallable, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Func {
    pub name: Option<String>,
    pub params: Vec<CandidType>,
    pub return_type: Box<ReturnType>,
    pub mode: Mode,
    pub to_vm_value: fn(String) -> TokenStream,
    pub list_to_vm_value: fn(String) -> TokenStream,
    pub from_vm_value: fn(String) -> TokenStream,
    pub list_from_vm_value: fn(String) -> TokenStream,
}

#[derive(Clone, Debug)]
pub enum Mode {
    Query,
    Update,
    Oneway,
}

impl Func {
    pub fn new(
        name: Option<String>,
        params: Vec<CandidType>,
        return_type: CandidType,
        mode: Mode,
        to_vm_value: fn(String) -> TokenStream,
        list_to_vm_value: fn(String) -> TokenStream,
        from_vm_value: fn(String) -> TokenStream,
        list_from_vm_value: fn(String) -> TokenStream,
    ) -> Func {
        Func {
            name,
            params,
            return_type: Box::new(ReturnType::new(return_type)),
            mode,
            to_vm_value,
            list_to_vm_value,
            from_vm_value,
            list_from_vm_value,
        }
    }

    fn get_name(&self, inline_name: String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => utils::create_inline_name(&inline_name),
        }
    }

    fn generate_func_struct_and_impls(
        &self,
        keyword_list: &Vec<String>,
        name: String,
    ) -> TokenStream {
        let type_alias_name = name.to_ident();
        let func_mode = match self.mode {
            Mode::Query => quote! {candid::parser::types::FuncMode::Query },
            Mode::Oneway => quote! {candid::parser::types::FuncMode::Oneway },
            Mode::Update => quote! {},
        };
        let param_type_strings: Vec<_> = self
            .params
            .iter()
            .enumerate()
            .map(|(index, candid_type)| {
                to_param(index, candid_type)
                    .to_type_annotation(keyword_list, name.clone())
                    .to_string()
            })
            .collect();
        let func_param_types: Vec<_> = param_type_strings
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
                        "Unable to parse parameter type {modified_rust_type} in Func {type_alias_name}"
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
            .as_ref()
            .clone()
            .to_type_annotation(keyword_list, name.clone())
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
                    "Unable to parse return type {return_type_string} in Func {type_alias_name}"
                ));
            quote! { #return_type_token_stream::_ty()}
        };

        let func_to_vm_value = (self.to_vm_value)(name.clone());
        let func_list_to_vm_value = (self.list_to_vm_value)(name.clone());
        let func_from_vm_value = (self.from_vm_value)(name.clone());
        let func_list_from_vm_value = (self.list_from_vm_value)(name.clone());

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

impl<C> ToTypeAnnotation<C> for Func {
    fn to_type_annotation(&self, _: &C, inline_name: String) -> TypeAnnotation {
        self.get_name(inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Vec<String>> for Func {
    fn to_declaration(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Option<Declaration> {
        Some(self.generate_func_struct_and_impls(keyword_list, self.get_name(inline_name)))
    }

    fn collect_inline_declarations(
        &self,
        keyword_list: &Vec<String>,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(inline_name), keyword_list)
    }
}

fn to_param(index: usize, candid_type: &CandidType) -> Param {
    Param {
        name: index.to_string(),
        candid_type: candid_type.clone(),
    }
}

impl IsCallable for Func {
    fn get_params(&self) -> Vec<Param> {
        self.params
            .iter()
            .enumerate()
            .map(|(index, candid_type)| to_param(index, candid_type))
            .collect()
    }

    fn get_return_type(&self) -> Option<ReturnType> {
        Some(self.return_type.as_ref().clone())
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
