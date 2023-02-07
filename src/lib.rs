use act::node::full_declaration::Declaration;
use act::node::full_declaration::ToDeclaration;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use std::fmt;

pub use act::actable::Actable;
pub use act::actable::ToActDataType;
pub use act::AbstractCanisterTree;

pub mod act;
pub mod generators;
pub mod keyword;
pub mod traits;

#[derive(Clone)]
pub enum CanisterMethodType {
    Heartbeat,
    Init,
    InspectMessage,
    PostUpgrade,
    PreUpgrade,
    Query,
    Update,
}

impl fmt::Display for CanisterMethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CanisterMethodType::Heartbeat => write!(f, "Heartbeat"),
            CanisterMethodType::Init => write!(f, "Init"),
            CanisterMethodType::InspectMessage => write!(f, "InspectMessage"),
            CanisterMethodType::PostUpgrade => write!(f, "PostUpgrade"),
            CanisterMethodType::PreUpgrade => write!(f, "PreUpgrade"),
            CanisterMethodType::Query => write!(f, "Query"),
            CanisterMethodType::Update => write!(f, "Update"),
        }
    }
}
pub enum RequestType {
    Query,
    Update,
}

pub enum SystemStructureType {
    Canister,
}

pub trait ToAct {
    fn to_act(&self) -> AbstractCanisterTree;
}

impl<C, T> ToDeclaration<C> for Option<T>
where
    T: ToDeclaration<C>,
{
    fn create_code(&self, context: &C, parental_prefix: String) -> Option<TokenStream> {
        match self {
            Some(t) => t.create_code(context, format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        match self {
            Some(t) => t.create_identifier(format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn create_child_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, Declaration> {
        match self {
            Some(t) => t.create_child_declarations(context, format!("{}Optional", parental_prefix)),
            None => HashMap::new(),
        }
    }
}
