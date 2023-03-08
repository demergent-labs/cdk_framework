fn snake_to_camel(s: &String) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }

    result
}

pub(crate) fn create_inline_name(base_name: &String) -> String {
    format!("_Inline{base}", base = snake_to_camel(base_name))
}
