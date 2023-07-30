use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_randomness_implementation() -> TokenStream {
    quote! {
        fn rng_seed() {
            ic_cdk::spawn(async move {
                let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::management_canister::main::raw_rand().await;

                match result {
                    Ok(randomness) => unsafe { ic_wasi_polyfill::init_seed(&randomness.0) },
                    Err(err) => panic!(err)
                };
            });
        }
    }
}
