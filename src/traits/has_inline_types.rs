use crate::act::{node::Context, Declaration};

use super::Declare;

pub trait HasInlines<T>
where
    T: Declare<Context>,
{
    fn get_inlines(&self) -> Vec<T>;

    fn flatten_inlines(
        &self,
        parent_name: String,
        context: &Context,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        self.get_inlines().iter().fold(vec![], |acc, item| {
            vec![acc, item.flatten(context, parent_name.clone(), module_name)].concat()
        })
    }
}
