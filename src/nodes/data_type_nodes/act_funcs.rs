use super::{ActDataType, HasMembers, LiteralOrTypeAlias, ToIdent, TypeAliasize};
use crate::ToTokenStream;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct ActFunc {
    pub act_type: LiteralOrTypeAlias<FuncLiteral, FuncTypeAlias>,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub name: String,
    pub params: Vec<ActDataType>,
    pub return_type: Box<Option<ActDataType>>,
    pub mode: String,
    pub to_vm_value: TokenStream,
    pub list_to_vm_value: TokenStream,
    pub from_vm_value: TokenStream,
    pub list_from_vm_value: TokenStream,
}

#[derive(Clone, Debug)]
pub struct FuncLiteral {
    pub func: Func,
}

#[derive(Clone, Debug)]
pub struct FuncTypeAlias {
    pub func: Func,
}

impl TypeAliasize<ActFunc> for ActFunc {
    fn as_type_alias(&self) -> ActFunc {
        match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => ActFunc {
                act_type: LiteralOrTypeAlias::TypeAlias(FuncTypeAlias {
                    func: literal.func.clone(),
                }),
            },
            LiteralOrTypeAlias::TypeAlias(_) => self.clone(),
        }
    }
}

impl HasMembers for ActFunc {
    fn get_members(&self) -> Vec<ActDataType> {
        let act_func = match &self.act_type {
            LiteralOrTypeAlias::Literal(literal) => &literal.func,
            LiteralOrTypeAlias::TypeAlias(type_alias) => &type_alias.func,
        };
        let return_type = match &*act_func.return_type {
            Some(return_type) => vec![return_type.clone()],
            None => vec![],
        };
        vec![act_func.params.clone(), return_type].concat()
    }
}

impl<C> ToTokenStream<C> for FuncLiteral {
    fn to_token_stream(&self, _: C) -> TokenStream {
        self.func.name.to_identifier().to_token_stream()
    }
}

impl ToTokenStream<&Vec<String>> for FuncTypeAlias {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        generate_func_struct_and_impls(&self.func, keyword_list)
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

fn generate_func_struct_and_impls(func: &Func, context: &Vec<String>) -> TokenStream {
    let type_alias_name = func.name.to_identifier();
    let func_mode = if func.mode == "Query" {
        quote! {candid::parser::types::FuncMode::Query }
    } else if func.mode == "Oneway" {
        quote! {candid::parser::types::FuncMode::Oneway }
    } else {
        quote! {}
    };
    let param_type_strings: Vec<String> = func
        .params
        .iter()
        .map(|param| param.to_token_stream(context).to_string())
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

            quote! {#modified_rust_type_token_stream::_ty()}
        })
        .collect();
    let return_type_string = match &*func.return_type {
        Some(return_type) => return_type.to_token_stream(context).to_string(),
        None => "".to_string(),
    };
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

    let func_to_vm_value = &func.to_vm_value;
    let func_list_to_vm_value = &func.list_to_vm_value;
    let func_from_vm_value = &func.from_vm_value;
    let func_list_from_vm_value = &func.list_from_vm_value;

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
