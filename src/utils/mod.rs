pub mod aliases;
mod argument_processor;
pub mod errors;
pub mod names;
mod run_backend;
pub mod typed_parse;

pub use argument_processor::ArgumentProcessor;
pub use names::provider::NamesProvider;
pub use run_backend::run_backend;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

pub fn path_argument(path: Option<Path>) -> TokenStream {
    path.map_or(TokenStream::new(), |path| {
        quote! {
            path = #path,
        }
    })
}
