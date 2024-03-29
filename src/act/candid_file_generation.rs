use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_candid_file_generation_code() -> TokenStream {
    quote! {
        candid::export_service!();

        // Heavily inspired by https://stackoverflow.com/a/47676844
        #[no_mangle]
        pub fn get_candid_pointer() -> *mut std::os::raw::c_char {
            let c_string = std::ffi::CString::new(__export_service()).unwrap();

            c_string.into_raw()
        }
    }
}
