use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        node::{node_parts::mode::Mode, Context, Param, ReturnType},
        Declaration,
    },
    traits::{HasInlineName, ToTypeAnnotation},
};

use super::{Declare, HasInlines};

#[derive(Clone)]
pub enum ParamOrReturn {
    ReturnType(ReturnType),
    Param(Param),
}

pub trait IsCallable {
    fn get_params(&self) -> Vec<Param>;
    fn get_return_type(&self) -> Option<ReturnType>;

    fn create_parameter_list_token_stream(
        &self,
        function_name: &String,
        context: &Context,
        module_name_option: &Option<String>,
    ) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .map(|param| param.to_token_stream(context, function_name.clone(), module_name_option))
            .collect();
        quote!(#(#params),*)
    }

    fn get_func_macro_token_stream(
        &self,
        function_name: &String,
        context: &Context,
        mode: &Mode,
        module_name: &Option<String>,
    ) -> TokenStream {
        let params_type_annotations =
            self.get_params_type_annotations(function_name, context, module_name);
        let return_type_annotation = self.get_return_type().unwrap().to_type_annotation(
            context,
            function_name.to_string(),
            module_name,
        );
        let func_mode = match mode {
            Mode::Query => quote!(query),
            Mode::Oneway => quote!(oneway),
            Mode::Update => quote!(),
        };

        quote! {
            (#params_type_annotations) -> (#return_type_annotation) #func_mode
        }
    }

    fn get_params_type_annotations(
        &self,
        function_name: &String,
        context: &Context,
        module_name: &Option<String>,
    ) -> TokenStream {
        let params: Vec<_> = self
            .get_params()
            .iter()
            .map(|param| {
                param.candid_type.to_type_annotation(
                    context,
                    param.get_inline_name(function_name),
                    module_name,
                )
            })
            .collect();
        quote!(#(#params),*)
    }
}

impl<T> HasInlines<ParamOrReturn> for T
where
    T: IsCallable,
{
    fn get_inlines(&self) -> Vec<ParamOrReturn> {
        vec![
            self.get_params()
                .into_iter()
                .map(|param| ParamOrReturn::Param(param))
                .collect::<Vec<_>>(),
            vec![self.get_return_type()]
                .into_iter()
                .filter_map(|x| match x {
                    Some(x) => Some(ParamOrReturn::ReturnType(x)),
                    None => None,
                })
                .collect(),
        ]
        .concat()
    }
}

impl Declare<Context> for ParamOrReturn {
    fn to_declaration(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        match &self {
            ParamOrReturn::ReturnType(return_type) => {
                return_type.to_declaration(context, function_name, module_name)
            }
            ParamOrReturn::Param(param) => {
                param.to_declaration(context, function_name, module_name)
            }
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        function_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        match &self {
            ParamOrReturn::ReturnType(return_type) => {
                return_type.collect_inline_declarations(context, function_name, module_name)
            }
            ParamOrReturn::Param(param) => {
                param.collect_inline_declarations(context, function_name, module_name)
            }
        }
    }
}
