use proc_macro2::TokenStream;

pub type Declaration = TokenStream;

pub trait Declare<C> {
    fn to_declaration(&self, context: &C, parental_prefix: String) -> Option<Declaration>;
    fn collect_inline_declarations(&self, context: &C, parental_prefix: String)
        -> Vec<Declaration>;

    fn flatten(&self, context: &C, parental_prefix: String) -> Vec<Declaration> {
        let declaration =
            if let Some(declaration) = self.to_declaration(context, parental_prefix.clone()) {
                vec![declaration]
            } else {
                vec![]
            };
        vec![
            declaration,
            self.collect_inline_declarations(context, parental_prefix),
        ]
        .concat()
    }
}
