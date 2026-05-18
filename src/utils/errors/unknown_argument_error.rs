use crate::utils::aliases::SynError;
use proc_macro2::Ident;

pub struct UnknownArgumentError;

impl UnknownArgumentError {
    pub fn new(name: Ident) -> SynError {
        SynError::new(name.span(), format!("Unknown argument `{name}`"))
    }
}
