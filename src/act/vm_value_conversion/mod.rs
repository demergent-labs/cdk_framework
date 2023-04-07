use proc_macro2::TokenStream;
pub use vm_value_conversion::VmValueConversion;

pub mod try_from_vm_value;
pub mod try_into_vm_value;
pub mod vm_value_conversion;

pub fn generate(vm_value_conversion: &VmValueConversion) -> TokenStream {
    let try_into_vm_value_trait = try_into_vm_value::generate_try_into_vm_value();
    let try_into_vm_value_impls = &vm_value_conversion.try_into_vm_value_impls;
    let try_from_vm_value_trait = try_from_vm_value::generate_try_from_vm_value();
    let try_from_vm_value_impls = &vm_value_conversion.try_from_vm_value_impls;

    quote::quote! {
        pub mod vm_value_conversion {
            use slotmap::Key as _SlotMapKeyTrait;
            use std::str::FromStr as _FromStrTrait;

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls
        }
    }
}
