use proc_macro2::TokenStream;
use quote::quote;

pub fn generate_try_into_vm_value() -> TokenStream {
    quote! {
        pub trait CdkActTryIntoVmValue<Context, VmValue> {
            fn try_into_vm_value(self, context: Context) -> std::result::Result<VmValue, CdkActTryIntoVmValueError>;
        }

        #[derive(Debug)]
        pub struct CdkActTryIntoVmValueError(pub String);
    }
}
