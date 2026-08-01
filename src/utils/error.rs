use crate::utils::aliases::{ProcMacroCrateError, SynError};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Crate `{0}` not found")]
    CrateNotFound(String),
    #[error("Duplicate argument `{0}`")]
    DuplicateArgument(Ident),
    #[error("Locale variants must be placed before named arguments")]
    LocaleVariantPosition(Ident),
    #[error("Expected `,` between arguments")]
    NoCommaBetweenArgument(Span),
    #[error("Expected at least one locale variant")]
    NoLocaleVariant,
    #[error("Missing required argument: `{0}`")]
    RequiredArgument(&'static str),
    #[error("Expected `{name}` to be an `{expected_type}`")]
    Type {
        span: Span,
        name: String,
        expected_type: &'static str,
    },
    #[error("Unknown argument `{0}`")]
    UnknownArgument(Ident),
    #[error(transparent)]
    Other(#[from] SynError),
}

impl Error {
    pub fn on_crate_not_found(name: &str) -> impl FnOnce(ProcMacroCrateError) -> Self {
        move |_| Self::CrateNotFound(name.to_string())
    }

    pub fn on_type_error(name: &str, expected_type: &'static str) -> impl FnOnce(SynError) -> Self {
        move |error| Self::Type {
            span: error.span(),
            name: name.to_string(),
            expected_type,
        }
    }
}

impl From<Error> for SynError {
    fn from(error: Error) -> SynError {
        match error {
            Error::Other(error) => error,
            error => {
                let span = match &error {
                    Error::CrateNotFound(_)
                    | Error::RequiredArgument(_)
                    | Error::NoLocaleVariant => Span::call_site(),
                    Error::DuplicateArgument(ident)
                    | Error::LocaleVariantPosition(ident)
                    | Error::UnknownArgument(ident) => ident.span(),
                    Error::NoCommaBetweenArgument(span) | Error::Type { span, .. } => *span,
                    Error::Other(_) => unreachable!(),
                };

                SynError::new(span, error)
            }
        }
    }
}

impl From<Error> for TokenStream {
    fn from(error: Error) -> Self {
        SynError::from(error).into_compile_error().into()
    }
}
