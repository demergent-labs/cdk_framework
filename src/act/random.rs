use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_randomness_implementation() -> TokenStream {
    quote! {
        fn rng_seed() {
            ic_cdk::spawn(async move {
                let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::call::call(
                    candid::Principal::from_text("aaaaa-aa").unwrap(),
                    "raw_rand",
                    ()
                ).await;

                match result {
                    Ok(randomness) => ic_wasi_polyfill::init(u64::from_be_bytes(randomness.0[..8].try_into().unwrap())),
                    Err(err) => panic!(err)
                };
            });
        }
    }
}
