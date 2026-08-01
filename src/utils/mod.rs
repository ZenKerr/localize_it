pub mod aliases;
mod argument_processor;
mod error;
pub mod names;
pub mod typed_parse;

pub use argument_processor::ArgumentProcessor;
pub use names::provider::NamesProvider;

use crate::utils::aliases::SynResult;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Path, bracketed,
    parse::{ParseBuffer, ParseStream},
};

pub fn path_argument(path: Option<Path>) -> TokenStream {
    path.map_or(TokenStream::new(), |path| {
        quote! {
            path = #path,
        }
    })
}

pub fn bracketed(stream: ParseStream) -> SynResult<ParseBuffer> {
    let content;
    bracketed!(content in stream);

    Ok(content)
}
