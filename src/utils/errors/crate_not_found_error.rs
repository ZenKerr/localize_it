use crate::utils::aliases::{ProcMacroCrateError, SynError};
use proc_macro2::Span;

pub struct CrateNotFoundError;

impl CrateNotFoundError {
    pub fn new(name: &str) -> SynError {
        SynError::new(Span::call_site(), format!("Crate `{name}` not found"))
    }

    pub fn map(name: &str) -> impl FnOnce(ProcMacroCrateError) -> SynError {
        move |_| Self::new(name)
    }
}
