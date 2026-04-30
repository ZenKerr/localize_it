use proc_macro2::TokenStream;
use quote::quote;
use syn::Path;

pub fn path_argument(path: Option<Path>) -> TokenStream {
    path.map_or(TokenStream::new(), |path| {
        quote! {
            path = #path
        }
    })
}
