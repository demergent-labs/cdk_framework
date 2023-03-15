pub trait HasInlineName {
    fn get_inline_name(&self, parent_name: &String) -> String;
}
