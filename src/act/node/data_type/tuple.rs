use super::{traits::HasMembers, DataType};
use crate::{traits::ToIdent, ToDeclarationTokenStream, ToTokenStream};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub name: String,
    pub elems: Vec<Elem>,
}

#[derive(Clone, Debug)]
pub struct Elem {
    pub elem_type: DataType,
}

impl HasMembers for Tuple {
    fn get_members(&self) -> Vec<DataType> {
        self.elems
            .iter()
            .map(|elem| elem.elem_type.clone())
            .collect()
    }
}

impl<C> ToTokenStream<C> for Tuple {
    fn to_token_stream(&self, _: C) -> TokenStream {
        self.name.to_identifier().to_token_stream()
    }
}

impl ToDeclarationTokenStream<&Vec<String>> for Tuple {
    fn to_declaration(&self, keyword_list: &Vec<String>, _: String) -> TokenStream {
        let type_ident = self.name.to_identifier();
        let elem_idents: Vec<TokenStream> = self
            .elems
            .iter()
            .map(|elem| elem.to_token_stream(keyword_list))
            .collect();

        let elem_idents = if elem_idents.len() == 1 {
            let elem_ident = &elem_idents[0];
            quote!((#elem_ident,))
        } else {
            quote!(#(#elem_idents),*)
        };

        quote!(
            #[derive(serde::Deserialize, Debug, candid::CandidType, Clone, CdkActTryIntoVmValue, CdkActTryFromVmValue)]
            struct #type_ident (
                #elem_idents
            );
        )
    }
}

impl ToTokenStream<&Vec<String>> for Elem {
    fn to_token_stream(&self, keyword_list: &Vec<String>) -> TokenStream {
        if self.elem_type.needs_to_be_boxed() {
            let ident = self.elem_type.to_token_stream(keyword_list);
            quote!(Box<#ident>)
        } else {
            quote!(self.elem_type.to_token_stream())
        }
    }
}
