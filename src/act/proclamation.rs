use super::Declaration;

#[derive(Clone)]
pub struct Proclamation {
    pub identifier: Option<String>,
    pub declaration: Option<Declaration>,
    pub inline_declarations: Vec<Declaration>,
}

pub trait Proclaim<C> {
    fn create_proclamation(&self, context: &C, parental_prefix: String) -> Proclamation {
        Proclamation {
            identifier: self.create_identifier(parental_prefix.clone()),
            declaration: self.create_declaration(context, parental_prefix.clone()),
            inline_declarations: self
                .collect_inline_declarations(&context, parental_prefix.clone()),
        }
    }

    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<Declaration>;
    fn create_identifier(&self, parental_prefix: String) -> Option<String>;
    fn collect_inline_declarations(&self, context: &C, parental_prefix: String)
        -> Vec<Declaration>;
}
