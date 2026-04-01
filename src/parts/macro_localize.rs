use crate::{arguments::Arguments, names::Names};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_localize(arguments: &Arguments, names: &Names) -> TokenStream {
    let localize_ident = names.get_hashed_name("localize");

    let auto_locale = if arguments.storage {
        let storage_path = names.get_path("storage");

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
            #auto_locale

            ($expression: path $(as ($($arg: expr),* $(,)?))?, $locale: expr) => {
                $expression[$locale.to_usize()]$(($($arg),*))?
            };
        }

        pub use #localize_ident as localize;
    }
}
