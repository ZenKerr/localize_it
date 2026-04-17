use crate::{backends::init_locale::arguments::Arguments, utils::NamesProvider};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

pub fn macro_expressions_from_files(
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> Result<TokenStream, Error> {
    Ok(if cfg!(feature = "from_files") {
        let expressions_from_files_ident = names_provider.get_hashed_name("expressions_from_files");
        let localize_it_crate = names_provider.get_crate_name("localize_it")?;
        let path = arguments
            .path
            .clone()
            .map_or(TokenStream::new(), |path| quote! {path = #path});

        quote! {
            #[macro_export]
            macro_rules! #expressions_from_files_ident {
                (
                    {
                        $($locale: ident => $path: path),+ $(,)?
                    } => [
                        $($expression: ident $(: $expression_type: ty)?),+ $(,)?
                    ]
                ) => {
                    #localize_it_crate::__expressions_from_files!(
                        locales = [$(($locale, $path)),+],
                        expressions = [$(($expression, $($expression_type)?)),+],
                        #path
                    );
                };
            }

            pub use #expressions_from_files_ident as expressions_from_files;
        }
    } else {
        TokenStream::new()
    })
}
