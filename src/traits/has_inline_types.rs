use crate::act::Declaration;

use super::Declare;

pub trait HasInlines<T>
where
    T: Declare<Vec<String>>,
{
    fn get_inlines(&self) -> Vec<T>;

    fn flatten_inlines(&self, parent_name: String, keyword_list: &Vec<String>) -> Vec<Declaration> {
        self.get_inlines().iter().fold(vec![], |acc, item| {
            vec![acc, item.flatten(keyword_list, parent_name.clone())].concat()
        })
    }
}
