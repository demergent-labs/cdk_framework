use quote::{format_ident, quote};

pub fn generate_randomness_implementation(cdk_name: &String) -> proc_macro2::TokenStream {
    let random_function_name = format_ident!("_{}_custom_getrandom", cdk_name.to_lowercase());

    quote! {
        fn #random_function_name(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
            RNG_REF_CELL.with(|rng_ref_cell| {
                let mut rng = rng_ref_cell.borrow_mut();
                rng.fill(_buf);
            });

            Ok(())
        }

        getrandom::register_custom_getrandom!(#random_function_name);
    }
}
