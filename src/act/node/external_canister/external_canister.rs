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
pub struct ExternalCanister {
    pub name: String,
    pub methods: Vec<Method>,
}

impl AsNode for ExternalCanister {
    fn as_node(self) -> Node {
        Node::ExternalCanister(self)
    }
}

impl Declare<Context> for ExternalCanister {
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

        // TODO add the vec implementations
        // TODO get rid of bad unwraps and stuff
        // TODO add many examples to the tests

        Some(quote! {
            ic_cdk::export::candid::define_service!(#service_name : {
                #(#service_funcs),*
            });

            impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for #service_name {
                fn try_into_vm_value(self, context: &mut boa_engine::Context) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
                    Ok(context.eval(
                        format!(
                            "new {}(Principal.fromText(\"{}\"))",
                            stringify!(#service_name),
                            self.0.principal.to_string()
                        )
                    ).unwrap())
                }
            }

            impl CdkActTryFromVmValue<#service_name, &mut boa_engine::Context> for boa_engine::JsValue {
                fn try_from_vm_value(self, context: &mut boa_engine::Context) -> Result<#service_name, CdkActTryFromVmValueError> {
                    let js_object = self.as_object().unwrap();
                    let canister_id_js_value = js_object.get("canisterId", context).unwrap();
                    let canister_id_js_object = canister_id_js_value.as_object().unwrap();
                    let canister_id_to_string_js_value = canister_id_js_object.get("toText", context).unwrap();
                    let canister_id_to_string_js_object = canister_id_to_string_js_value.as_object().unwrap();
                    let canister_id_string_js_value = canister_id_to_string_js_object.call(
                        &canister_id_js_value,
                        &[],
                        context
                    ).unwrap();
                    let canister_id_js_string = canister_id_string_js_value.to_string(context).unwrap();
                    let canister_id_string = canister_id_js_string.to_std_string_escaped();

                    Ok(#service_name::new(ic_cdk::export::Principal::from_str(&canister_id_string).unwrap()))
                }
            }

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
