use crate::utils::aliases::SynError;
use proc_macro2::Ident;

pub struct LocaleVariantPositionError;

impl LocaleVariantPositionError {
    pub fn new(argument: Ident) -> SynError {
        SynError::new(
            argument.span(),
            "Locale variants must be placed before named arguments",
        )
    }
}
