use proc_macro2::TokenStream;
use quote::quote;

pub fn generate() -> TokenStream {
    quote! {
        #[derive(serde::Deserialize, Clone, Debug, candid::CandidType)]
        struct _CdkFloat32(f32);

        impl std::cmp::Ord for _CdkFloat32 {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
            }
        }

        impl std::cmp::PartialOrd for _CdkFloat32 {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl std::cmp::Eq for _CdkFloat32 {}

        impl std::cmp::PartialEq for _CdkFloat32 {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
    }
}
