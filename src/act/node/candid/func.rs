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
        let name = self.get_name(inline_name.clone()).to_ident();
        let params_type_annotations =
            self.get_params_type_annotations(&self.get_name(inline_name.clone()), keyword_list);

        // Because of the way void and null work with Candid/the Rust cdk, we need to do this check
        // Basically if we see the type annotation is () we need to treat it as an empty quote!()
        // So that the define_function macro doesn't pick it up as null
        let return_type_annotation = self
            .return_type
            .to_type_annotation(keyword_list, inline_name.clone());
        let return_type_annotation = if return_type_annotation.to_string() == "()" {
            quote!()
        } else {
            return_type_annotation
        };
        let func_mode = match self.mode {
            Mode::Query => quote!(query),
            Mode::Oneway => quote!(oneway),
            Mode::Update => quote!(),
        };

        let func_to_vm_value = (self.to_vm_value)(self.get_name(inline_name.clone()));
        let func_list_to_vm_value = (self.list_to_vm_value)(self.get_name(inline_name.clone()));
        let func_from_vm_value = (self.from_vm_value)(self.get_name(inline_name.clone()));
        let func_list_from_vm_value = (self.list_from_vm_value)(self.get_name(inline_name.clone()));

        Some(quote! {
            ic_cdk::export::candid::define_function!(pub #name : (#params_type_annotations) -> (#return_type_annotation) #func_mode);

            #func_to_vm_value
            #func_list_to_vm_value
            #func_from_vm_value
            #func_list_from_vm_value
        })
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
