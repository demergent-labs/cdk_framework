use proc_macro2::TokenStream;

pub struct VmValueConversions {
    pub try_from_vm_value_impls: TokenStream,
    pub try_into_vm_value_impls: TokenStream,
}
