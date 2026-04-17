use crate::{backends::init_locale::arguments::Arguments, utils::NamesProvider};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_localize(arguments: &Arguments, names_provider: &NamesProvider) -> TokenStream {
    let localize_ident = names_provider.get_hashed_name("localize");

    let locale_from_storage = if arguments.storage {
        let storage_path = names_provider.get_path("storage");

        quote! {
            ($expression: path $(as ($($arg: expr),* $(,)?))?) => {
                $expression[#storage_path::get_as_usize()]$(($($arg),*))?
            };
        }
    } else {
        TokenStream::new()
    };

    quote! {
        #[macro_export]
        macro_rules! #localize_ident {
            #locale_from_storage

            ($expression: path $(as ($($arg: expr),* $(,)?))?, $locale: expr) => {
                $expression[$locale.to_usize()]$(($($arg),*))?
            };
        }

        pub use #localize_ident as localize;
    }
}
