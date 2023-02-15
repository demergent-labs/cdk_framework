use act::proclamation::Proclaim;
use proc_macro2::TokenStream;
use std::collections::HashMap;
use std::fmt;

pub mod act;
pub mod generators;
pub mod keyword;
pub mod traits;

pub use act::to_node::ToDataType;
pub use act::to_node::ToNode;
pub use act::AbstractCanisterTree;

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

impl<C, T> Proclaim<C> for Option<T>
where
    T: Proclaim<C>,
{
    fn create_declaration(&self, context: &C, parental_prefix: String) -> Option<TokenStream> {
        match self {
            Some(t) => t.create_declaration(context, format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn create_identifier(&self, parental_prefix: String) -> Option<String> {
        match self {
            Some(t) => t.create_identifier(format!("{}Optional", parental_prefix)),
            None => None,
        }
    }

    fn collect_inline_declarations(
        &self,
        context: &C,
        parental_prefix: String,
    ) -> HashMap<String, TokenStream> {
        match self {
            Some(t) => {
                t.collect_inline_declarations(context, format!("{}Optional", parental_prefix))
            }
            None => HashMap::new(),
        }
    }
}
