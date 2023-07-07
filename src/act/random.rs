use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_randomness_implementation() -> TokenStream {
    quote! {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        thread_local! {
            static _CDK_RNG_REF_CELL: std::cell::RefCell<rand::rngs::StdRng> = std::cell::RefCell::new(rand::SeedableRng::from_seed([0u8; 32]));
        }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
            _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
                let mut rng = rng_ref_cell.borrow_mut();
                rng.fill(_buf);
            });

            Ok(())
        }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        getrandom::register_custom_getrandom!(custom_getrandom);

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        fn rng_seed() {
            ic_cdk::spawn(async move {
                let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::management_canister::main::raw_rand().await;

                _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
                    let mut rng = rng_ref_cell.borrow_mut();

                    match result {
                        Ok(randomness) => *rng = rand::SeedableRng::from_seed(randomness.0[..].try_into().unwrap()),
                        Err(err) => panic!(err)
                    };
                });
            });
        }

        #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
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
