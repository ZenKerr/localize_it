mod r#enum;
mod methods;
mod traits;

use crate::{
    backends::init_locale::{
        arguments::Arguments,
        parts::enum_locale::{methods::methods, r#enum::r#enum, traits::traits},
    },
    utils::{aliases::SynResult, names::ENUM_LOCALE, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn enum_locale(arguments: &Arguments) -> SynResult<TokenStream> {
    let locale_ident = NamesProvider::get_name(ENUM_LOCALE);

    let r#enum = r#enum(arguments);
    let methods = methods(arguments)?;
    let traits = traits()?;

    Ok(quote! {
        mod __locale {
            use super::*;

            #r#enum
            #methods
            #traits
        }

        pub use __locale::#locale_ident;
    })
}
