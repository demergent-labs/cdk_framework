use crate::traits::{ContainsNodeWithName, HasDefinedNames, HasTypeRefs};

use super::node::candid::{Func, Record, Service, Tuple, TypeAlias, TypeRef, Variant};

#[derive(Clone)]
pub struct CandidTypes {
    pub funcs: Vec<Func>,
    pub records: Vec<Record>,
    pub services: Vec<Service>,
    pub tuples: Vec<Tuple>,
    pub type_aliases: Vec<TypeAlias>,
    pub variants: Vec<Variant>,
}

impl HasTypeRefs for CandidTypes {
    fn get_type_refs(&self) -> Vec<TypeRef> {
        self.records
            .iter()
            .flat_map(|record| record.get_type_refs())
            .chain(self.variants.iter().flat_map(|v| v.get_type_refs()))
            .chain(self.tuples.iter().flat_map(|t| t.get_type_refs()))
            .chain(self.type_aliases.iter().flat_map(|t| t.get_type_refs()))
            .chain(self.services.iter().flat_map(|s| s.get_type_refs()))
            .chain(self.funcs.iter().flat_map(|f| f.get_type_refs()))
            .collect()
    }
}

impl HasDefinedNames for CandidTypes {
    fn get_defined_names(&self) -> Vec<String> {
        self.funcs
            .iter()
            .map(|f| f.name.clone())
            .chain(self.records.iter().map(|r| r.name.clone()))
            .chain(self.tuples.iter().map(|t| t.name.clone()))
            .chain(self.variants.iter().map(|v| v.name.clone()))
            .filter_map(|name| name)
            .chain(self.services.iter().map(|s| s.name.clone()))
            .chain(self.type_aliases.iter().map(|t| t.name.clone()))
            .collect()
    }
}

impl ContainsNodeWithName for CandidTypes {
    fn contains_node_with_name(&self, name: &str) -> bool {
        if self.funcs.iter().any(|func| opt_eq(&func.name, name)) {
            return true;
        }
        if self.records.iter().any(|record| opt_eq(&record.name, name)) {
            return true;
        }
        if self.services.iter().any(|service| service.name == name) {
            return true;
        }
        if self.tuples.iter().any(|tuple| opt_eq(&tuple.name, name)) {
            return true;
        }
        if self
            .type_aliases
            .iter()
            .any(|type_alias| type_alias.name == name)
        {
            return true;
        }
        if self
            .variants
            .iter()
            .any(|variant| opt_eq(&variant.name, name))
        {
            return true;
        }

        false
    }
}

fn opt_eq(option: &Option<String>, str: &str) -> bool {
    match option.as_deref() {
        Some(s) if s == str => true,
        _ => false,
    }
}
