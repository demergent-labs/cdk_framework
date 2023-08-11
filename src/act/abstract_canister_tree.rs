use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    act::{
        candid_file_generation, float32, float64, random, vm_value_conversion, CandidTypes,
        CanisterMethods, VmValueConversion,
        {
            node::{AsNode, CandidType, CanisterMethod, Context, GuardFunction},
            Declaration, Declare,
        },
    },
    traits::{ContainsNodeWithName, HasDefinedNames, HasTypeRefs, ToIdent},
};

use super::node::{
    candid::TypeRef,
    canister_method::{InitMethod, PostUpgradeMethod},
};

/// An easily traversable representation of a rust canister
pub struct AbstractCanisterTree {
    pub cdk_name: String,
    pub header: TokenStream,
    pub body: TokenStream,
    pub vm_value_conversion: VmValueConversion,
    pub keywords: Vec<String>,
    pub modules: Vec<Module>,
    pub default_init_method: InitMethod,
    pub default_post_upgrade_method: PostUpgradeMethod,
}

pub struct Module {
    pub path: Vec<String>,
    pub canister_methods: CanisterMethods,
    pub candid_types: CandidTypes,
    pub guard_functions: Vec<GuardFunction>,
    pub body: TokenStream,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

// TODO For import names we need to follow all renaming rules like as or whatever
#[derive(Debug)]
pub struct Import {
    pub names: Vec<String>,
    pub path: Vec<String>,
}

// TODO For export names we need to follow all renaming rules like as or whatever
#[derive(Debug)]
pub struct Export {
    pub names: Vec<String>,
    pub path: Vec<String>,
}

pub fn convert_module_path_to_name(path: &Vec<String>) -> String {
    path.join("_").to_string()
}

impl Module {
    fn collect_canister_methods(&self) -> Vec<CanisterMethod> {
        let init_method = match &self.canister_methods.init_method {
            Some(init_method) => Some(CanisterMethod::Init(init_method.clone())),
            None => None,
        };
        let heartbeat_method = match &self.canister_methods.heartbeat_method {
            Some(heartbeat_method) => Some(CanisterMethod::Heartbeat(heartbeat_method.clone())),
            None => None,
        };
        let inspect_message_method = match &self.canister_methods.inspect_message_method {
            Some(inspect_message_method) => Some(CanisterMethod::InspectMessage(
                inspect_message_method.clone(),
            )),
            None => None,
        };
        let pre_upgrade_method = match &self.canister_methods.pre_upgrade_method {
            Some(pre_upgrade_method) => {
                Some(CanisterMethod::PreUpgrade(pre_upgrade_method.clone()))
            }
            None => None,
        };
        let post_upgrade_method = match &self.canister_methods.post_upgrade_method {
            Some(post_upgrade_method) => {
                Some(CanisterMethod::PostUpgrade(post_upgrade_method.clone()))
            }
            None => None,
        };
        let system_canister_methods: Vec<_> = vec![
            init_method,
            heartbeat_method,
            inspect_message_method,
            pre_upgrade_method,
            post_upgrade_method,
        ]
        .iter()
        .filter_map(|system_canister_method| system_canister_method.clone())
        .collect();

        let query_methods = self
            .canister_methods
            .query_methods
            .iter()
            .map(|query| CanisterMethod::Query(query.clone()))
            .collect();
        let update_methods = self
            .canister_methods
            .update_methods
            .iter()
            .map(|update| CanisterMethod::Update(update.clone()))
            .collect();

        vec![system_canister_methods, query_methods, update_methods].concat()
    }

    fn collect_candid_types(&self) -> Vec<CandidType> {
        let funcs: Vec<_> = self
            .candid_types
            .funcs
            .iter()
            .map(|func| CandidType::Func(func.clone()))
            .collect();
        let records = self
            .candid_types
            .records
            .iter()
            .map(|record| CandidType::Record(record.clone()))
            .collect();
        let services = self
            .candid_types
            .services
            .iter()
            .map(|service| CandidType::Service(service.clone()))
            .collect();
        let tuples = self
            .candid_types
            .tuples
            .iter()
            .map(|tuple| CandidType::Tuple(tuple.clone()))
            .collect();
        let type_aliases = self
            .candid_types
            .type_aliases
            .iter()
            .map(|type_alias| CandidType::TypeAlias(type_alias.clone()))
            .collect();
        let variants = self
            .candid_types
            .variants
            .iter()
            .map(|variant| CandidType::Variant(variant.clone()))
            .collect();

        vec![funcs, records, services, tuples, type_aliases, variants].concat()
    }

    fn generate_imports(&self, act: &AbstractCanisterTree) -> Vec<TokenStream> {
        // use module_specifer::{list, of, imports}
        // let imports = self.imports.iter().filter(|import| act.contains_node_with_name(import))

        let deduplicated_imports = self.deduplicate_imports();

        deduplicated_imports
            .iter()
            .map(|import| import.generate_imports(act))
            .collect()
    }

    fn generate_exports(&self, act: &AbstractCanisterTree) -> Vec<TokenStream> {
        // use module_specifer::{list, of, imports}
        // let imports = self.imports.iter().filter(|import| act.contains_node_with_name(import))
        self.exports
            .iter()
            .map(|export| export.generate_exports(act))
            .collect()
    }

    // We cannot both import and export the same thing
    // If an import exists in the exports, then we will favor the export
    // because the export will achieve both purposes, importing and exporting
    // because of the pub use
    fn deduplicate_imports<'a>(&'a self) -> Vec<&'a Import> {
        self.imports
            .iter()
            .filter(|import| {
                !self.exports.iter().any(|export| {
                    export.names.iter().any(|export_name| {
                        import
                            .names
                            .iter()
                            .any(|import_name| export_name == import_name)
                    })
                })
            })
            .collect()
    }

    fn is_init_method_defined(&self) -> bool {
        self.canister_methods.init_method.is_some()
    }

    fn is_post_upgrade_method_defined(&self) -> bool {
        self.canister_methods.post_upgrade_method.is_some()
    }
}

impl Import {
    fn generate_imports(&self, act: &AbstractCanisterTree) -> TokenStream {
        let names: Vec<_> = self
            .names
            .iter()
            .filter(|import| act.contains_node_with_name(import))
            .map(|name| name.to_ident())
            .collect();

        if names.len() > 0 {
            let path = convert_module_path_to_name(&self.path).to_ident();
            quote! {pub use crate::#path::{#(#names),*};}
        } else {
            quote!()
        }
    }
}

impl Export {
    fn generate_exports(&self, act: &AbstractCanisterTree) -> TokenStream {
        let names: Vec<_> = self
            .names
            .iter()
            .filter(|import| act.contains_node_with_name(import))
            .map(|name| name.to_ident())
            .collect();

        if names.len() > 0 {
            let path = convert_module_path_to_name(&self.path).to_ident();
            quote! {pub use crate::#path::{#(#names),*};}
        } else {
            quote!()
        }
    }
}

pub enum Error {
    MultipleTypeDefinitions(String),
    MultipleGuardFunctionDefinitions(String),
    MultipleCanisterMethodDefinitions(String),
    TypeNotFound(String),
    GuardFunctionNotFound(String),
}

impl AbstractCanisterTree {
    pub fn to_token_stream(&self) -> Result<TokenStream, Vec<Error>> {
        // TODO these verifications need to be redone for all modules
        let errors = self
            .verify_type_refs_have_corresponding_definitions()
            .err()
            .into_iter()
            .chain(self.verify_type_defs_are_unique().err())
            .chain(
                self.verify_guard_function_names_have_corresponding_definitions()
                    .err(),
            )
            .chain(self.verify_guard_function_defs_are_unique().err())
            .chain(self.verify_canister_method_defs_are_unique().err())
            .flatten()
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            return Err(errors);
        }

        let header = &self.header;

        let randomness_implementation = random::generate_randomness_implementation();

        let try_into_vm_value_trait = vm_value_conversion::generate_try_into_vm_value();
        let try_into_vm_value_impls = &self.vm_value_conversion.try_into_vm_value_impls;
        let try_from_vm_value_trait = vm_value_conversion::generate_try_from_vm_value();
        let try_from_vm_value_impls = &self.vm_value_conversion.try_from_vm_value_impls;

        let body = &self.body;

        let modules = self.modules.iter().map(|module| {
            let module_name_string = convert_module_path_to_name(&module.path);
            let module_name_ident = module_name_string.to_ident();

            let canister_method_decls = self.generate_declarations(
                module.collect_canister_methods(),
                &Some(module_name_string),
            );
            let candid_type_decls =
                self.generate_declarations(module.collect_candid_types(), &None);
            let guard_function_decls =
                self.generate_declarations(module.guard_functions.clone(), &None);

            let body = &module.body;

            let imports = module.generate_imports(&self);
            let exports = module.generate_exports(&self);

            // TODO working on _AzleResult...see if we can resolve this thing entirely and get rid of it
            quote! {
                mod #module_name_ident {
                    use crate::CdkActTryIntoVmValue;
                    use crate::CdkActTryFromVmValue;
                    use crate::CdkActTryIntoVmValueError;
                    use crate::ToJsError;
                    use crate::unwrap_or_trap;
                    use crate::UnwrapJsResultOrTrap;
                    use crate::ToStdString;
                    use crate::UnwrapOrTrapWithMessage;

                    #(#imports)*

                    #(#exports)*

                    #(#canister_method_decls)*
                    #(#candid_type_decls)*
                    #(#guard_function_decls)*

                    #body
                }
            }
        });

        let candid_file_generation_code =
            candid_file_generation::generate_candid_file_generation_code();

        let azle_float64 = float64::generate();
        let azle_float32 = float32::generate();

        let default_init_method = if !self.is_init_method_defined() {
            Some(self.default_init_method.to_declaration(
                &self.build_context(),
                "".to_string(),
                &None,
            ))
        } else {
            None
        };

        let default_post_upgrade_method = if !self.is_post_upgrade_method_defined() {
            Some(self.default_post_upgrade_method.to_declaration(
                &self.build_context(),
                "".to_string(),
                &None,
            ))
        } else {
            None
        };

        Ok(quote! {
            #header

            #randomness_implementation

            #try_into_vm_value_trait
            #try_into_vm_value_impls
            #try_from_vm_value_trait
            #try_from_vm_value_impls

            #default_init_method

            #default_post_upgrade_method

            #body

            #(#modules)*

            #candid_file_generation_code

            #azle_float64
            #azle_float32
        })
    }

    fn generate_declarations<T: AsNode>(
        &self,
        list: Vec<T>,
        module_name: &Option<String>,
    ) -> Vec<Declaration> {
        list.into_iter().fold(vec![], |acc, t| {
            vec![
                acc,
                t.as_node()
                    .flatten(&self.build_context(), "".to_string(), module_name),
            ]
            .concat()
        })
    }

    fn build_context(&self) -> Context {
        Context {
            keyword_list: self.keywords.clone(),
            cdk_name: self.cdk_name.clone(),
        }
    }

    fn verify_type_defs_are_unique(&self) -> Result<(), Vec<Error>> {
        let defined_names = self.modules[0].candid_types.get_defined_names();
        let duplicates = find_duplicates(&defined_names);

        match duplicates.is_empty() {
            true => Ok(()),
            false => Err(duplicates
                .into_iter()
                .map(|type_ref| Error::MultipleTypeDefinitions(type_ref.clone()))
                .collect()),
        }
    }

    fn verify_guard_function_defs_are_unique(&self) -> Result<(), Vec<Error>> {
        let defined_names = self.modules[0].guard_functions.get_defined_names();
        let duplicates = find_duplicates(&defined_names);

        match duplicates.is_empty() {
            true => Ok(()),
            false => Err(duplicates
                .into_iter()
                .map(|type_ref| Error::MultipleGuardFunctionDefinitions(type_ref.clone()))
                .collect()),
        }
    }

    fn verify_canister_method_defs_are_unique(&self) -> Result<(), Vec<Error>> {
        let defined_names = self.modules[0].canister_methods.get_defined_names();
        let duplicates = find_duplicates(&defined_names);

        match duplicates.is_empty() {
            true => Ok(()),
            false => Err(duplicates
                .into_iter()
                .map(|type_ref| Error::MultipleCanisterMethodDefinitions(type_ref.clone()))
                .collect()),
        }
    }

    fn verify_type_refs_have_corresponding_definitions(&self) -> Result<(), Vec<Error>> {
        let defined_names: HashSet<_> = self.modules[0]
            .candid_types
            .get_defined_names()
            .into_iter()
            .collect();
        let used_names: HashSet<_> = self
            .get_type_refs()
            .iter()
            .map(|type_ref| type_ref.name.clone())
            .collect();

        let diff: Vec<_> = used_names.difference(&defined_names).cloned().collect();

        match diff.is_empty() {
            true => Ok(()),
            false => Err(diff
                .iter()
                .map(|type_ref| Error::TypeNotFound(type_ref.clone()))
                .collect()),
        }
    }

    fn verify_guard_function_names_have_corresponding_definitions(&self) -> Result<(), Vec<Error>> {
        let defined_names_set: HashSet<_> = self.modules[0]
            .guard_functions
            .iter()
            .map(|f| f.name.clone())
            .collect();
        let used_guard_functions: HashSet<_> = self.modules[0]
            .canister_methods
            .collect_used_guard_function_names()
            .into_iter()
            .collect();

        let diff: Vec<_> = used_guard_functions
            .difference(&defined_names_set)
            .cloned()
            .collect();

        match diff.is_empty() {
            true => Ok(()),
            false => Err(diff
                .iter()
                .map(|type_ref| Error::GuardFunctionNotFound(type_ref.clone()))
                .collect()),
        }
    }

    fn is_init_method_defined(&self) -> bool {
        self.modules
            .iter()
            .any(|module| module.is_init_method_defined())
    }

    fn is_post_upgrade_method_defined(&self) -> bool {
        self.modules
            .iter()
            .any(|module| module.is_post_upgrade_method_defined())
    }
}

impl HasTypeRefs for AbstractCanisterTree {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.modules[0]
            .canister_methods
            .get_type_refs()
            .into_iter()
            .chain(self.modules[0].candid_types.get_type_refs())
            .collect()
    }
}

fn find_duplicates<T: Eq + std::hash::Hash>(list: &[T]) -> Vec<&T> {
    let count_map = list.iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });

    count_map
        .iter()
        .filter(|(_, &count)| count > 1)
        .map(|(&item, _)| item)
        .collect()
}

impl ContainsNodeWithName for Module {
    fn contains_node_with_name(&self, name: &str) -> bool {
        if self.canister_methods.contains_node_with_name(name) {
            return true;
        }
        if self.candid_types.contains_node_with_name(name) {
            return true;
        }
        if self
            .guard_functions
            .iter()
            .any(|guard_function| guard_function.name == name)
        {
            return true;
        }
        false
    }
}

impl ContainsNodeWithName for AbstractCanisterTree {
    fn contains_node_with_name(&self, name: &str) -> bool {
        self.modules
            .iter()
            .any(|module| module.contains_node_with_name(name))
    }
}
