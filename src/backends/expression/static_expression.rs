use crate::{
    backends::expression::arguments::Arguments,
    utils::{names::ENUM_LOCALE, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn static_expression(argument: &Arguments, names_provider: &NamesProvider) -> TokenStream {
    let locale_path = names_provider.get_path(ENUM_LOCALE);
    let name = &argument.name;
    let r#type = &argument.r#type;
    let locales = &argument.locales;
    let values = &argument.values;

    quote! {
        pub static #name: [#r#type; #locale_path::COUNT] = {
            let mut expression = [#(#values),*];
            let mut empty = [true; #locale_path::COUNT];

            #(
                let i = #locale_path::#locales.to_usize();

                if core::mem::replace(&mut empty[i], false) {
                    expression[i] = #values;
                } else {
                    panic!(concat!(
                        "Locale variant ",
                        stringify!(locales),
                        " is duplicated"
                    ));
                }
            )*

            expression
        };
    }
}
