use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::Elem;
use crate::{
    act::{
        node::{candid::type_param::TypeParams, Context, Member},
        Declaration, Declare, ToTypeAnnotation, TypeAnnotation,
    },
    traits::{HasInlines, HasMembers, ToIdent},
    utils,
};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub name: Option<String>,
    pub elems: Vec<Elem>,
    pub type_params: TypeParams,
}

impl Tuple {
    fn get_name(&self, inline_name: &String) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => utils::create_inline_name(&inline_name),
        }
    }
}

impl<C> ToTypeAnnotation<C> for Tuple {
    fn to_type_annotation(
        &self,
        _: &C,
        inline_name: String,
        module_name_option: &Option<String>,
    ) -> TypeAnnotation {
        let name = self.get_name(&inline_name).to_ident().to_token_stream();

        let module_name_ident = if let Some(module_name) = module_name_option {
            Some(module_name.to_string().to_ident())
        } else {
            None
        };

        quote!(crate::#module_name_ident::#name)
    }
}

impl Declare<Context> for Tuple {
    fn to_declaration(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Option<Declaration> {
        let tuple_ident = self.get_name(&inline_name).to_ident();
        let member_idents: Vec<TokenStream> = self
            .elems
            .iter()
            .enumerate()
            .map(|(index, elem)| {
                elem.to_tuple_elem_token_stream(
                    index,
                    &self.get_name(&inline_name),
                    context,
                    module_name,
                )
            })
            .collect();

        let member_idents = if member_idents.len() == 1 {
            let member_ident = &member_idents[0];
            quote!((#member_ident,))
        } else {
            quote!(#(#member_idents),*)
        };

        let type_params_token_stream = self.type_params.get_type_params_token_stream();
        let where_clause_token_stream = self.type_params.get_where_clause_token_stream();

        Some(quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue, Ord, PartialOrd, Eq, PartialEq)]
            pub struct #tuple_ident #type_params_token_stream (
                #member_idents
            ) #where_clause_token_stream;
        ))
    }

    fn collect_inline_declarations(
        &self,
        context: &Context,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.flatten_inlines(self.get_name(&inline_name), context, module_name)
    }
}

impl HasMembers for Tuple {
    fn get_members(&self) -> Vec<Member> {
        self.elems
            .iter()
            .enumerate()
            .map(|(index, elem)| elem.to_member(index))
            .collect()
    }

    fn get_type_params(&self) -> TypeParams {
        self.type_params.clone()
    }
}
