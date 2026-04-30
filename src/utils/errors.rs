use proc_macro2::{Ident, Span};
use syn::Error;

pub fn type_error(name: &str, expected_type: &str) -> impl FnOnce(Error) -> Error {
    move |error: Error| {
        Error::new(
            error.span(),
            format!("Expected `{name}` to be an `{expected_type}`"),
        )
    }
}

pub fn required_argument_error(name: &str) -> Error {
    Error::new(
        Span::call_site(),
        format!("Missing required argument: `{name}`"),
    )
}

pub fn unknown_argument_error(name: Ident) -> Error {
    Error::new(name.span(), format!("Unknown argument `{name}`"))
}
