use std::ops::Deref;

use super::{QueryMethod, QueryOrUpdateDefinition, UpdateMethod};

#[derive(Clone, Debug)]
pub enum QueryOrUpdateMethod {
    Query(QueryMethod),
    Update(UpdateMethod),
}

impl Deref for QueryOrUpdateMethod {
    type Target = QueryOrUpdateDefinition;

    fn deref(&self) -> &Self::Target {
        match self {
            QueryOrUpdateMethod::Query(query) => &query.definition,
            QueryOrUpdateMethod::Update(update) => &update.definition,
        }
    }
}
