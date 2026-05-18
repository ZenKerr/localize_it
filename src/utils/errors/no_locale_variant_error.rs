use crate::utils::aliases::SynError;
use proc_macro2::Span;

pub struct NoLocaleVariantError;

impl NoLocaleVariantError {
    pub fn new() -> SynError {
        SynError::new(Span::call_site(), "Expected at least one locale variant")
    }
}
