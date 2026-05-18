use crate::utils::aliases::SynError;
use proc_macro2::Ident;

pub struct DuplicateArgumentError;

impl DuplicateArgumentError {
    pub fn new(name: &Ident) -> SynError {
        SynError::new(name.span(), format!("Duplicate argument `{name}`"))
    }
}
