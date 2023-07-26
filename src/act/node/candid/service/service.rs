use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::Method;
use crate::{
    act::{
        node::{candid::TypeRef, Context},
        Declaration, Declare, TypeAnnotation,
    },
    traits::{HasTypeRefs, IsCallable, ToIdent, ToTypeAnnotation},
};

#[derive(Clone, Debug)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
    pub to_vm_value: fn(String) -> TokenStream,
    pub list_to_vm_value: fn(String) -> TokenStream,
    pub from_vm_value: fn(String) -> TokenStream,
    pub list_from_vm_value: fn(String) -> TokenStream,
}

impl Service {
    fn get_name(&self, _: &String) -> String {
        self.name.clone()
    }
}

impl ToTypeAnnotation<Context> for Service {
    fn to_type_annotation(&self, _: &Context, inline_name: String) -> TypeAnnotation {
        self.get_name(&inline_name).to_ident().to_token_stream()
    }
}

impl Declare<Context> for Service {
    fn to_declaration(&self, context: &Context, _: String) -> Option<Declaration> {
        let cross_canister_call_functions: Vec<_> = self
            .methods
            .iter()
            .filter_map(|method| method.to_declaration(context, self.name.clone()))
            .collect();

        let service_name = self.name.to_ident();
        let service_funcs: Vec<_> = self
            .methods
            .iter()
            .map(|method| {
                let method_name = method.name.clone();
                let func_macro_token_stream = method.get_func_macro_token_stream(
                    &method.create_qualified_name(&self.name),
                    context,
                    &method.mode,
                );

                quote! {
                    #method_name: candid::func!(#func_macro_token_stream)
                }
            })
            .collect();

        let service_to_vm_value = (self.to_vm_value)(self.name.clone());
        let service_list_to_vm_value = (self.list_to_vm_value)(self.name.clone());
        let service_from_vm_value = (self.from_vm_value)(self.name.clone());
        let service_list_from_vm_value = (self.list_from_vm_value)(self.name.clone());

        Some(quote! {
            candid::define_service!(#service_name : {
                #(#service_funcs);*
            });

            impl std::cmp::Ord for #service_name {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
                }
            }

            impl std::cmp::PartialOrd for #service_name {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    None
                }
            }

            #service_to_vm_value
            #service_list_to_vm_value
            #service_from_vm_value
            #service_list_from_vm_value

            #(#cross_canister_call_functions)*
        })
    }

    fn collect_inline_declarations(&self, context: &Context, _: String) -> Vec<Declaration> {
        self.methods.iter().fold(vec![], |acc, method| {
            vec![
                acc,
                method.collect_inline_declarations(context, self.name.clone()),
            ]
            .concat()
        })
    }
}

impl HasTypeRefs for Service {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.methods
            .iter()
            .flat_map(|method| method.get_type_refs())
            .collect()
    }
}
