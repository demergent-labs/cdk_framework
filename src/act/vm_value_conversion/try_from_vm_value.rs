use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_try_from_vm_value() -> TokenStream {
    quote! {
        pub trait CdkActTryFromVmValue<T, Context> {
            fn try_from_vm_value(self, context: Context) -> Result<T, CdkActTryFromVmValueError>;
        }

        #[derive(Debug)]
        pub struct CdkActTryFromVmValueError(pub String);
    }
}
