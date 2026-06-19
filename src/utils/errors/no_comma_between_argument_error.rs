use crate::utils::aliases::SynError;
use syn::parse::ParseStream;

pub struct NoCommaBetweenArgumentError;

impl NoCommaBetweenArgumentError {
    pub fn new(parse_stream: ParseStream) -> SynError {
        parse_stream.error("Expected `,` between argument")
    }
}
