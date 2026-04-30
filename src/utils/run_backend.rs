use proc_macro::TokenStream;
use syn::Result;

pub fn run_backend(
    backend_function: fn(TokenStream) -> Result<TokenStream>,
    input: TokenStream,
) -> TokenStream {
    backend_function(input).unwrap_or_else(|error| error.to_compile_error().into())
}
