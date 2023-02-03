use proc_macro2::TokenStream;
use quote::quote;
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

pub trait ToTokenStream<C> {
    fn to_token_stream(&self, context: C) -> TokenStream;
}

pub trait ToDeclarationTokenStream<C> {
    fn to_declaration(&self, context: C, parental_prefix: String) -> TokenStream;
}

impl<C, T> ToDeclarationTokenStream<C> for Vec<T>
where
    C: Clone,
    T: ToDeclarationTokenStream<C>,
{
    fn to_declaration(&self, context: C, parental_prefix: String) -> TokenStream {
        let declarations = self.iter().enumerate().map(|(index, t)| {
            t.to_declaration(context.clone(), format!("{}{}", parental_prefix, index))
        });
        quote!(#(#declarations)*)
    }
}

pub trait ToTokenStreams<C> {
    fn to_token_streams(&self, context: C) -> Vec<TokenStream>;
}

impl<C, T> ToTokenStreams<C> for Vec<T>
where
    C: Clone,
    T: ToTokenStream<C>,
{
    fn to_token_streams(&self, context: C) -> Vec<TokenStream> {
        self.iter()
            .map(|t| t.to_token_stream(context.clone()))
            .collect()
    }
}

impl<C, T> ToTokenStream<C> for Vec<T>
where
    C: Clone,
    T: ToTokenStream<C>,
{
    fn to_token_stream(&self, context: C) -> TokenStream {
        let declarations = self.iter().map(|t| t.to_token_stream(context.clone()));
        quote!(#(#declarations)*)
    }
}
impl<C, T> ToTokenStream<C> for Option<T>
where
    T: ToTokenStream<C>,
{
    fn to_token_stream(&self, context: C) -> TokenStream {
        match self {
            Some(t) => t.to_token_stream(context),
            None => quote::quote! {},
        }
    }
}
