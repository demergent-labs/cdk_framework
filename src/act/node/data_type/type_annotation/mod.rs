use proc_macro2::TokenStream;

pub type TypeAnnotation = TokenStream;

pub trait ToTypeAnnotation<C> {
    fn to_type_annotation(&self, context: &C, parental_prefix: String) -> TypeAnnotation;
}
