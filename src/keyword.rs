use regex::Regex;

pub fn restore(name: &String, keywords: &Vec<String>) -> String {
    let keyword_list = to_regex_list(keywords);

    let matches = keyword_list.iter().fold(false, |acc, keyword_regex| {
        acc || keyword_regex.is_match(name)
    });

    if matches {
        name[..name.len() - 1].to_string()
    } else {
        name.clone()
    }
}

pub fn to_regex_list(keywords: &Vec<String>) -> Vec<Regex> {
    keywords
        .iter()
        .map(|keyword| {
            let keyword = regex::escape(keyword);
            Regex::new(format!(r#"^{}_+$"#, keyword).as_str()).unwrap()
        })
        .collect()
}
