use proc_macro2::{Ident, TokenStream};
use quote::quote;
use regex::Regex;

const RUST_KEYWORDS: [&str; 51] = [
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in",
    "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// Why do we need this?
///
/// All of the languages that need to be serialized into candid have different
/// sets of keyword. A name that would be a valid record or variant member name
/// in language 'A' might be a reserved keyword in language 'B'. So we need a
/// method for language 'B' to interact with a candid record from language 'A'.
///
/// This issue is further complicated for this cdk framework in particular since
/// it uses Rust as an intermediate language so a name that might be valid in
/// both language 'A' and language 'B' might be a reserved keyword in Rust and
/// therefore would not compile properly with this cdk framework.
///
/// How do we fix it?
///
/// Our solution is to append an underscore to the end of any problematic names.
/// For writing in language 'B' the user will add underscores themselves. We
/// will then find those keywords and restore them when they are serialized to
/// candid so that the underscore will not show up in the candid file. In the
/// event the name they want to write ends with an underscore in the candid they
/// they will add a second underscore and we will remove only the last
/// underscore and thus leave the one needed underscore. For example if 'from'
/// was a keyword in language 'B' the user will write 'from_' in their
/// application. If they need to express 'from_' then they will write 'from__'
/// etc.
///
/// For rust keywords, we don't want to bother the user with the details of our
/// in-between language so we will let them write those keywords normally. We
/// will intercept those keywords when we are transpiling from language 'B' to
/// rust and append the underscore automatically for them.
///
/// We then need to intercept those names again before they are fed into the vm,
/// and as they are coming out of the vm and back into our rust environment, and
/// remove/append the underscore respectively so that the vm will recognize the
/// names but rust won't choke on them.
///
/// How does it work?
///
/// When we write the Variants and Record in this CDK we will first check the
/// member name to see if it is a rust keyword and if so make it rust safe by
/// appending the underscore by running the [make_rust_safe] function, then if
/// it was a rust keyword or if it is a keyword from current language with an
/// underscore appended to it, we will add the serde rename attribute, so that
/// it will be encoded into candid without the extra underscore that we or the
/// user added using the [generate_rename_attribute] function.
///
/// The developer using this cdk framework will be responsible for handling
/// values as they go into and out of the vm. This will need to be done in their
/// implementation of the derive_try_from_vm_value and derive_try_into_vm_value
/// for structs and enums. They will need to use the [restore_for_vm] function
/// to handle any potential keyword conversions.

pub fn generate_rename_attribute(name: &Ident, keyword_list: &Vec<String>) -> TokenStream {
    let restored_keyword = restore_for_serde(&name.to_string(), keyword_list);
    if restored_keyword == name.to_string() {
        quote!()
    } else {
        quote! {
           #[serde(rename = #restored_keyword)]
        }
    }
}

// To generate a rust safe keyword we need to look at the keyword. If it matches
// any rust keywords followed by zero or more underscores then we need to make
// it rust safe by appending one additional underscore.
// But if it matches any language specific keywords then we shouldn't add
// another one.
pub fn make_rust_safe(name: &String, keywords: &Vec<String>) -> String {
    if !is_language_safe_keyword(name, keywords) && is_rust_unsafe_keyword(name) {
        format!("{}_", name)
    } else {
        name.clone()
    }
}

// Remove the ending underscore if the name is a rust keyword and it isn't a
// language specific keyword. If the name is a language specific keyword then
// the vm will need the underscore and we must not remove it.
pub fn restore_for_vm(name: &String, keywords: &Vec<String>) -> String {
    if !is_language_safe_keyword(name, keywords) && is_rust_safe_keyword(name) {
        name[..name.len() - 1].to_string()
    } else {
        name.clone()
    }
}

// Remove the ending underscore if the name is a rust keyword and is a language
// specific keyword. Serde should never need extra underscores.
fn restore_for_serde(name: &String, keywords: &Vec<String>) -> String {
    if is_language_safe_keyword(name, keywords) || is_rust_safe_keyword(name) {
        name[..name.len() - 1].to_string()
    } else {
        name.clone()
    }
}

fn is_rust_unsafe_keyword(name: &String) -> bool {
    RUST_KEYWORDS.iter().fold(false, |acc, keyword| {
        acc || to_unsafe_regex(&keyword.to_string()).is_match(name)
    })
}

fn is_rust_safe_keyword(name: &String) -> bool {
    RUST_KEYWORDS.iter().fold(false, |acc, keyword| {
        acc || to_safe_regex(&keyword.to_string()).is_match(name)
    })
}

fn is_language_safe_keyword(name: &String, keywords: &Vec<String>) -> bool {
    keywords.iter().fold(false, |acc, keyword| {
        acc || to_safe_regex(keyword).is_match(name)
    })
}

// Convert name to a regex that checks if it is a safe keyword to use
// Safe means that it is suffixed with at least one underscore
fn to_safe_regex(name: &String) -> Regex {
    let name = regex::escape(name);
    Regex::new(format!(r#"^{}_+$"#, name).as_str()).unwrap()
}

// Convert name to a regex that checks if it is an unsafe keyword to use
// Unsafe means that it may not have been suffixed with an underscore
// unsafe: from and from_ safe versions of those from_ and from__
fn to_unsafe_regex(name: &String) -> Regex {
    let name = regex::escape(name);
    Regex::new(format!(r#"^{}_*$"#, name).as_str()).unwrap()
}
