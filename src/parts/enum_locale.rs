use crate::{arguments::Arguments, names::Names};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::LitStr;

pub fn enum_locale(arguments: &Arguments, names: &Names) -> TokenStream {
    let locale_ident = names.get_name("Locale");
    let default = &arguments.default;
    let derive = &arguments.derive;
    let variants_number = arguments.variants.len();
    let (variant_names, variant_labels) = arguments
        .variants
        .iter()
        .map(|variant| (&variant.name, &variant.label))
        .unzip::<&Ident, &LitStr, Vec<&Ident>, Vec<&LitStr>>();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, #(#derive),*)]
        #[repr(usize)]
        pub enum #locale_ident {
            #(#variant_names),*
        }

        impl #locale_ident {
            pub const COUNT: usize = #variants_number;
            pub const VARIANTS: [Self; Self::COUNT] = [#(Self::#variant_names),*];
            pub const LABELS: [&'static str; Self::COUNT] = [#(#variant_labels),*];
            pub const DEFAULT: Self = Self::#default;

            #[inline]
            pub fn iter() -> impl Iterator<Item = (Self, &'static str)> {
                Self::iter_variants().zip(Self::iter_labels())
            }

            #[inline]
            pub fn iter_variants() -> impl Iterator<Item = Self> {
                Self::VARIANTS.into_iter()
            }

            #[inline]
            pub fn iter_labels() -> impl Iterator<Item = &'static str> {
                Self::LABELS.into_iter()
            }

            #[inline]
            pub const fn to_usize(self) -> usize {
                self as usize
            }

            #[inline]
            pub const fn from_usize(value: usize) -> Option<Self> {
                match value {
                    #(
                        _ if value == Self::#variant_names.to_usize() => Some(Self::#variant_names)
                    ),*,
                    _ => None,
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
                        Self::#variant_names => stringify!(#variant_names)
                    ),*
                }
            }

            #[inline]
            pub fn from_str(str: &str) -> Option<Self> {
                match str {
                    #(
                        stringify!(#variant_names) => Some(Self::#variant_names)
                    ),*,
                    _ => None,
                }
            }

            #[inline]
            pub fn from_str_or_default(str: &str) -> Self {
                Self::from_str(str).unwrap_or_default()
            }

            #[inline]
            pub const fn from_caseless_str(str: &str) -> Option<Self> {
                match str {
                    #(
                        _ if str.eq_ignore_ascii_case(stringify!(#variant_names)) => Some(Self::#variant_names)
                    ),*,
                    _ => None,
                }
            }

            #[inline]
            pub fn from_caseless_str_or_default(str: &str) -> Self {
                Self::from_caseless_str(str).unwrap_or_default()
            }
        }

        impl Default for #locale_ident {
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

        impl From<#locale_ident> for usize {
            #[inline]
            fn from(locale: #locale_ident) -> Self {
                locale.to_usize()
            }
        }

        impl TryFrom<usize> for #locale_ident {
            type Error = &'static str;

            #[inline]
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Self::from_usize(value).ok_or("Invalid locale identifier")
            }
        }

        impl From<#locale_ident> for &str {
            #[inline]
            fn from(locale: #locale_ident) -> Self {
                locale.to_str()
            }
        }

        impl core::str::FromStr for #locale_ident {
            type Err = &'static str;

            #[inline]
            fn from_str(str: &str) -> Result<Self, Self::Err> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }

        impl TryFrom<&str> for #locale_ident {
            type Error = &'static str;

            #[inline]
            fn try_from(str: &str) -> Result<Self, Self::Error> {
                Self::from_str(str).ok_or("Invalid locale identifier")
            }
        }
    }
}
