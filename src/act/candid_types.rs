use super::node::candid::{Func, Record, Service, Tuple, TypeAlias, Variant};

pub struct CandidTypes {
    pub funcs: Vec<Func>,
    pub records: Vec<Record>,
    pub services: Vec<Service>,
    pub tuples: Vec<Tuple>,
    pub type_aliases: Vec<TypeAlias>,
    pub variants: Vec<Variant>,
}
