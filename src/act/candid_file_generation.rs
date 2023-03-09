use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_candid_file_generation_code() -> TokenStream {
    quote! {
        candid::export_service!();

        // Heavily inspired by https://stackoverflow.com/a/47676844
        use std::ffi::CString;
        use std::os::raw::c_char;

        #[no_mangle]
        pub fn _cdk_get_candid_pointer() -> *mut c_char {
            let c_string = CString::new(__export_service()).unwrap();

            c_string.into_raw()
        }

        #[no_mangle]
        pub fn _cdk_get_candid_length() -> usize {
            __export_service().len()
        }
    }
}
