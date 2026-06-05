use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{aliases::SynResult, names::ENUM_LOCALE, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn methods(arguments: &Arguments) -> SynResult<TokenStream> {
    let locale_ident = NamesProvider::get_name(ENUM_LOCALE);
    let default = &arguments.default;
    let variants = &arguments.variants;
    let variants_label = &arguments.variants_label;
    let variants_number = arguments.variants.len();

    let iterator_path = NamesProvider::get_path("core::iter::Iterator")?;
    let option_path = NamesProvider::get_path("core::option::Option")?;
    let some_path = NamesProvider::get_path("core::option::Option::Some")?;
    let none_path = NamesProvider::get_path("core::option::Option::None")?;

    Ok(quote! {
        impl #locale_ident {
            pub const COUNT: usize = #variants_number;
            pub const VARIANTS: [Self; Self::COUNT] = [#(Self::#variants),*];
            pub const LABELS: [&'static str; Self::COUNT] = [#(#variants_label),*];
            pub const DEFAULT: Self = Self::#default;

            #[inline]
            pub fn iter() -> impl #iterator_path<Item = (Self, &'static str)> {
                Self::iter_variants().zip(Self::iter_labels())
            }

            #[inline]
            pub fn iter_variants() -> impl #iterator_path<Item = Self> {
                Self::VARIANTS.into_iter()
            }

            #[inline]
            pub fn iter_labels() -> impl #iterator_path<Item = &'static str> {
                Self::LABELS.into_iter()
            }

            #[inline]
            pub const fn to_usize(self) -> usize {
                self as usize
            }

            #[inline]
            pub const fn from_usize(value: usize) -> #option_path<Self> {
                match value {
                    #(
                        _ if value == Self::#variants.to_usize() => #some_path(Self::#variants)
                    ),*,
                    _ => #none_path,
                }
            }

            #[inline]
            pub fn from_usize_or_default(value: usize) -> Self {
                Self::from_usize(value).unwrap_or_default()
            }

            #[inline]
            pub const fn to_str(self) -> &'static str {
                match self {
                    #(
                        Self::#variants => stringify!(#variants)
                    ),*
                }
            }

            #[inline]
            pub fn from_str(str: &str) -> #option_path<Self> {
                match str {
                    #(
                        stringify!(#variants) => #some_path(Self::#variants)
                    ),*,
                    _ => #none_path,
                }
            }

            #[inline]
            pub fn from_str_or_default(str: &str) -> Self {
                Self::from_str(str).unwrap_or_default()
            }

            #[inline]
            pub const fn from_caseless_str(str: &str) -> #option_path<Self> {
                match str {
                    #(
                        _ if str.eq_ignore_ascii_case(stringify!(#variants)) => #some_path(Self::#variants)
                    ),*,
                    _ => #none_path,
                }
            }

            #[inline]
            pub fn from_caseless_str_or_default(str: &str) -> Self {
                Self::from_caseless_str(str).unwrap_or_default()
            }
        }
    })
}
