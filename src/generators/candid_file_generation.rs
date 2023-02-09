use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_candid_file_generation_code(cdk_name: &String) -> TokenStream {
    let function_name = format_ident!("_{}_export_candid", cdk_name.to_lowercase());
    let test_function_name = format_ident!("_{}_write_candid_to_disk", cdk_name.to_lowercase());

    quote! {
        candid::export_service!();

        #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
        fn #function_name() -> String {
            __export_service()
        }

        #[cfg(test)]

        mod tests {
            use super::*;

            #[test]
            fn #test_function_name() {
                std::fs::write("index.did", #function_name()).unwrap();
            }
        }
    }
}
