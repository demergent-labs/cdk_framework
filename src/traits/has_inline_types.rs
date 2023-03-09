use crate::act::Declaration;

use super::{Declare, HasPrefix};

pub trait HasInlines<T>
where
    T: Declare<Vec<String>> + HasPrefix,
{
    fn get_inlines(&self) -> Vec<T>;

    fn flatten_inlines(&self, parent_name: String, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_inlines().iter().fold(vec![], |acc, item| {
            vec![
                acc,
                item.flatten(keyword_list, item.get_prefix(&parent_name)),
            ]
            .concat()
        })
    }
}
