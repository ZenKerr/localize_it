use crate::utils::aliases::SynError;
use proc_macro2::Span;

pub struct RequiredArgumentError;

impl RequiredArgumentError {
    pub fn new(name: &str) -> SynError {
        SynError::new(
            Span::call_site(),
            format!("Missing required argument: `{name}`"),
        )
    }
}
