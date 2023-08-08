use proc_macro2::TokenStream;

pub type Declaration = TokenStream;

pub trait Declare<C> {
    fn to_declaration(
        &self,
        context: &C,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Option<Declaration>;
    fn collect_inline_declarations(
        &self,
        context: &C,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration>;

    fn flatten(
        &self,
        context: &C,
        inline_name: String,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        let declaration = if let Some(declaration) =
            self.to_declaration(context, inline_name.clone(), module_name)
        {
            vec![declaration]
        } else {
            vec![]
        };
        vec![
            declaration,
            self.collect_inline_declarations(context, inline_name, module_name),
        ]
        .concat()
    }
}
