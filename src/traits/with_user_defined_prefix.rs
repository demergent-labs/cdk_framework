use std::fmt::Display;

pub trait WithUserDefinedPrefix {
    fn with_user_defined_prefix(&self) -> String
    where
        Self: Display,
    {
        format!("_cdk_user_defined_{self}")
    }
}

impl WithUserDefinedPrefix for String {}
