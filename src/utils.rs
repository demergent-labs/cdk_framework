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

    capitalize(result)
}

fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub(crate) fn create_inline_name(base_name: &String) -> String {
    let base_name = snake_to_camel(base_name);
    if base_name.starts_with("Inline") {
        return format!("_{base_name}");
    }
    format!("_Inline{base_name}")
}
