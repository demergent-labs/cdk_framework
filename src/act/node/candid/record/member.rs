use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{node::Context, ToTypeAnnotation},
    keyword,
    traits::ToIdent,
};

use super::{Member, Record};

impl Member {
    pub fn to_record_member_token_stream(
        &self,
        context: &Context,
        parent: &Record,
        inline_name: &str,
        module_name: &Option<String>,
    ) -> TokenStream {
        let type_annotation = self.to_type_annotation(
            context,
            parent.get_name(&inline_name.to_string()),
            module_name,
        );
        let name = keyword::make_rust_safe(&self.name, &context.keyword_list).to_ident();
        let rename_attr = keyword::generate_rename_attribute(&name, &context.keyword_list);
        quote!(pub #rename_attr #name: Box<#type_annotation>)
    }
}
