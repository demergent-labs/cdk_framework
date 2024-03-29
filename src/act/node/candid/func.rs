use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::{
    act::{
        node::{canister_method, node_parts::mode::Mode, CandidType, Context, Param, ReturnType},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasInlines, HasTypeRefs, IsCallable, ToIdent},
    utils,
};

use super::TypeRef;

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
}

impl<C> ToTypeAnnotation<C> for Func {
    fn to_type_annotation(&self, _: &C, inline_name: String) -> TypeAnnotation {
        self.get_name(inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Context> for Func {
    fn to_declaration(&self, context: &Context, inline_name: String) -> Option<Declaration> {
        let name = self.get_name(inline_name.clone()).to_ident();
        let func_macro_token_stream =
            self.get_func_macro_token_stream(&inline_name, context, &self.mode);

        let func_to_vm_value = (self.to_vm_value)(self.get_name(inline_name.clone()));
        let func_list_to_vm_value = (self.list_to_vm_value)(self.get_name(inline_name.clone()));
        let func_from_vm_value = (self.from_vm_value)(self.get_name(inline_name.clone()));
        let func_list_from_vm_value = (self.list_from_vm_value)(self.get_name(inline_name.clone()));

        Some(quote! {
            candid::define_function!(pub #name : #func_macro_token_stream);

            #func_to_vm_value
            #func_list_to_vm_value
            #func_from_vm_value
            #func_list_from_vm_value

            impl std::cmp::Ord for #name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
                }
            }

            impl std::cmp::PartialOrd for #name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    None
                }
            }
        })
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(inline_name), context)
    }
}

fn to_param(index: usize, candid_type: &CandidType) -> Param {
    Param {
        name: format!("Param{}", index.to_string()),
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

impl HasTypeRefs for Func {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        canister_method::get_type_refs(&self.get_params(), Some(&self.return_type))
    }
}
