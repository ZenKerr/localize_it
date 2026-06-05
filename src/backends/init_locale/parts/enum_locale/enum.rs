use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{names::ENUM_LOCALE, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn r#enum(arguments: &Arguments) -> TokenStream {
    let locale_ident = NamesProvider::get_name(ENUM_LOCALE);
    let variants = &arguments.variants;
    let derive = &arguments.derive;

    quote! {
        #[derive(
            core::fmt::Debug,
            core::clone::Clone,
            core::marker::Copy,
            core::cmp::PartialEq,
            core::cmp::Eq,
            core::cmp::PartialOrd,
            core::cmp::Ord,
            core::hash::Hash,
            #(#derive),*
        )]
        #[repr(usize)]
        pub enum #locale_ident {
            #(#variants),*
        }
    }
}
