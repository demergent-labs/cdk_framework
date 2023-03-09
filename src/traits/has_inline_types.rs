use crate::act::Declaration;

use super::{Declare, HasPrefix};

pub trait HasInlineTypes<T>
where
    T: Declare<Vec<String>> + HasPrefix,
{
    fn get_inline_items(&self) -> Vec<T>;

    fn collect_inline_declarations_from(
        &self,
        parent_name: String,
        keyword_list: &Vec<String>,
    ) -> Vec<Declaration> {
        self.get_inline_items().iter().fold(vec![], |acc, item| {
            vec![
                acc,
                item.flatten(keyword_list, item.get_prefix(&parent_name)),
            ]
            .concat()
        })
    }
}
