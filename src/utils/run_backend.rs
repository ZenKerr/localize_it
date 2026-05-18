use crate::utils::aliases::SynResult;
use proc_macro::TokenStream;

pub fn run_backend(
    backend_function: fn(TokenStream) -> SynResult<TokenStream>,
    input: TokenStream,
) -> TokenStream {
    backend_function(input).unwrap_or_else(|error| error.into_compile_error().into())
}
