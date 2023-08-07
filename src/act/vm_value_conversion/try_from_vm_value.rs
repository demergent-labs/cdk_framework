use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_try_from_vm_value() -> TokenStream {
    quote! {
        pub trait CdkActTryFromVmValue<Ok, Err, Context> {
            fn try_from_vm_value(self, context: Context) -> Result<Ok, Err>;
        }
    }
}
