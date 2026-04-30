use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{names::MACRO_EXPRESSIONS_FROM_FILES, path_argument, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn macro_expressions_from_files(
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> Result<TokenStream> {
    Ok(if cfg!(feature = "from_files") {
        let expressions_from_files_ident = names_provider.get_name(MACRO_EXPRESSIONS_FROM_FILES);
        let expressions_from_files_hashed_ident =
            names_provider.get_hashed_name(MACRO_EXPRESSIONS_FROM_FILES);
        let expressions_from_files_path = names_provider.get_path(MACRO_EXPRESSIONS_FROM_FILES);
        let localize_it_crate = names_provider.get_crate_name("localize_it")?;

        let path_argument = path_argument(arguments.path.clone());

        quote! {
            #[macro_export]
            macro_rules! #expressions_from_files_hashed_ident {
                (@type_or_default) => {
                    &'static str
                };

                (@type_or_default $r#type: ty) => {
                    $r#type
                };

                (
                    {
                        $($locale: ident => $path: path),+ $(,)?
                    } => [
                        $($value: ident $(: $r#type: ty)?),+ $(,)?
                    ]
                ) => {
                    #localize_it_crate::__expressions_from_files!(
                        locales = [$($locale),+],
                        locales_path = [$($path),+],
                        expressions = [$($value),+],
                        expressions_type = [$(#expressions_from_files_path!(@type_or_default $($r#type)?)),+],
                        #path_argument
                    );
                };
            }

            pub use #expressions_from_files_hashed_ident as #expressions_from_files_ident;
        }
    } else {
        TokenStream::new()
    })
}
