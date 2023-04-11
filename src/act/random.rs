use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_randomness_implementation() -> TokenStream {
    quote! {
        thread_local! {
            static _CDK_RNG_REF_CELL: std::cell::RefCell<rand::rngs::StdRng> = std::cell::RefCell::new(rand::SeedableRng::from_seed([0u8; 32]));
        }

        fn _cdk_custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
            _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
                let mut rng = rng_ref_cell.borrow_mut();
                rng.fill(_buf);
            });

            Ok(())
        }

        getrandom::register_custom_getrandom!(_cdk_custom_getrandom);

        fn _cdk_rng_seed() {
            ic_cdk::spawn(async move {
                let result: ic_cdk::api::call::CallResult<(Vec<u8>,)> = ic_cdk::api::call::call(
                    candid::Principal::from_text("aaaaa-aa").unwrap(),
                    "raw_rand",
                    ()
                ).await;

                _CDK_RNG_REF_CELL.with(|rng_ref_cell| {
                    let mut rng = rng_ref_cell.borrow_mut();

                    match result {
                        Ok(randomness) => *rng = rand::SeedableRng::from_seed(randomness.0[..].try_into().unwrap()),
                        Err(err) => panic!(err)
                    };
                });
            });
        }
    }
}
