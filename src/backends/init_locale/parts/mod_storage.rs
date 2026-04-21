use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{
        names::{ENUM_LOCALE, MOD_STORAGE},
        NamesProvider,
    },
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn mod_storage(arguments: &Arguments, names_provider: &NamesProvider) -> TokenStream {
    let storage_ident = names_provider.get_name(MOD_STORAGE);
    let locale_ident = names_provider.get_name(ENUM_LOCALE);

    if arguments.storage {
        quote! {
            pub mod #storage_ident {
                use super::#locale_ident;
                use core::sync::atomic::{AtomicUsize, Ordering};

                static CURRENT_LOCALE: AtomicUsize = AtomicUsize::new(#locale_ident::DEFAULT.to_usize());

                #[inline]
                pub fn get() -> #locale_ident {
                    #locale_ident::from_usize_or_default(CURRENT_LOCALE.load(Ordering::Relaxed))
                }

                #[inline]
                pub fn set(locale: #locale_ident) {
                    CURRENT_LOCALE.store(locale.to_usize(), Ordering::Relaxed);
                }

                #[inline]
                pub fn get_as_usize() -> usize {
                    get().to_usize()
                }

                #[inline]
                pub fn set_from_usize(value: usize) -> Result<(), &'static str> {
                    #locale_ident::from_usize(value)
                        .map(set)
                        .ok_or("Invalid locale identifier")
                }

                #[inline]
                pub fn set_from_usize_or_default(value: usize) {
                    set(#locale_ident::from_usize_or_default(value));
                }

                #[inline]
                pub fn get_as_str() -> &'static str {
                    get().to_str()
                }

                #[inline]
                pub fn set_from_str(str: &str) -> Result<(), &'static str> {
                    #locale_ident::from_str(str)
                        .map(set)
                        .ok_or("Invalid locale identifier")
                }

                #[inline]
                pub fn set_from_str_or_default(str: &str) {
                    set(#locale_ident::from_str_or_default(str));
                }

                #[inline]
                pub fn set_from_caseless_str(str: &str) -> Result<(), &'static str> {
                    #locale_ident::from_caseless_str(str)
                        .map(set)
                        .ok_or("Invalid locale identifier")
                }

                #[inline]
                pub fn set_from_caseless_str_or_default(str: &str) {
                    set(#locale_ident::from_caseless_str_or_default(str));
                }

                #[inline]
                pub fn reset() {
                    set(#locale_ident::DEFAULT);
                }
            }
        }
    } else {
        TokenStream::new()
    }
}
