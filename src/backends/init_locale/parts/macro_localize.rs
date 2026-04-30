use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{
        names::{MACRO_LOCALIZE, MOD_STORAGE},
        NamesProvider,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_localize(arguments: &Arguments, names_provider: &NamesProvider) -> TokenStream {
    let localize_ident = names_provider.get_name(MACRO_LOCALIZE);
    let localize_hashed_ident = names_provider.get_hashed_name(MACRO_LOCALIZE);

    let locale_from_storage = if arguments.storage {
        let storage_path = names_provider.get_path(MOD_STORAGE);

        quote! {
            ($expression: path $(as ($($argument: expr),* $(,)?))?) => {
                $expression[#storage_path::get_as_usize()]$(($($argument),*))?
            };
        }
    } else {
        TokenStream::new()
    };

    quote! {
        #[macro_export]
        macro_rules! #localize_hashed_ident {
            #locale_from_storage

            ($expression: path $(as ($($argument: expr),* $(,)?))?, $locale: expr) => {
                $expression[$locale.to_usize()]$(($($argument),*))?
            };
        }

        pub use #localize_hashed_ident as #localize_ident;
    }
}
