use crate::utils::aliases::SynError;

pub struct TypeError;

impl TypeError {
    pub fn new(error: SynError, name: &str, expected_type: &str) -> SynError {
        SynError::new(
            error.span(),
            format!("Expected `{name}` to be an `{expected_type}`"),
        )
    }

    pub fn map(name: &str, expected_type: &str) -> impl FnOnce(SynError) -> SynError {
        move |error| Self::new(error, name, expected_type)
    }
}
