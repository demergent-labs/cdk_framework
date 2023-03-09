pub trait HasPrefix {
    fn get_prefix(&self, parent_name: &String) -> String;
}
