use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{aliases::SynResult, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn traits(arguments: &Arguments) -> SynResult<TokenStream> {
    let locale_ident = NamesProvider::get_name(&arguments.locale_name);

    let from_path = NamesProvider::get_path("core::convert::From")?;
    let try_from_path = NamesProvider::get_path("core::convert::TryFrom")?;
    let result_path = NamesProvider::get_path("core::result::Result")?;

    Ok(quote! {
        impl core::default::Default for #locale_ident {
            #[inline]
            fn default() -> Self {
                Self::DEFAULT
            }
        }

        impl core::fmt::Display for #locale_ident {
            #[inline]
            fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str(self.to_str())
            }
        }

        impl #from_path<#locale_ident> for usize {
            #[inline]
            fn from(locale: #locale_ident) -> Self {
                locale.to_usize()
            }
        }

        impl #try_from_path<usize> for #locale_ident {
            type Error = &'static str;

            #[inline]
            fn try_from(value: usize) -> #result_path<Self, Self::Error> {
                Self::from_usize(value).ok_or("Invalid locale identifier")
            }
        }

        impl #from_path<#locale_ident> for &str {
            #[inline]
            fn from(locale: #locale_ident) -> Self {
                locale.to_str()
            }
        }

        impl core::str::FromStr for #locale_ident {
            type Err = &'static str;

            #[inline]
            fn from_str(str: &str) -> #result_path<Self, Self::Err> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }

        impl #try_from_path<&str> for #locale_ident {
            type Error = &'static str;

            #[inline]
            fn try_from(str: &str) -> #result_path<Self, Self::Error> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }
    })
}
