use proc_macro2::TokenStream;
use quote::quote;

use super::Method;
use crate::{
    act::{
        node::{AsNode, Context, Node},
        Declaration, Declare,
    },
    traits::{IsCallable, ToIdent},
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

impl AsNode for Service {
    fn as_node(self) -> Node {
        Node::Service(self)
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
                    &self.name,
                    &self.name,
                    &context.keyword_list,
                    &method.mode,
                );

                quote! {
                    #method_name: ic_cdk::export::candid::func!(#func_macro_token_stream)
                }
            })
            .collect();

        let service_to_vm_value = (self.to_vm_value)(self.name.clone());
        let service_list_to_vm_value = (self.list_to_vm_value)(self.name.clone());
        let service_from_vm_value = (self.from_vm_value)(self.name.clone());
        let service_list_from_vm_value = (self.list_from_vm_value)(self.name.clone());

        Some(quote! {
            ic_cdk::export::candid::define_service!(#service_name : {
                #(#service_funcs);*
            });

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
