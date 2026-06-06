use crate::{backends::expression::arguments::Arguments, utils::NamesProvider};
use proc_macro2::TokenStream;
use quote::quote;

pub fn static_expression(arguments: &Arguments, names_provider: &NamesProvider) -> TokenStream {
    let locale_path = names_provider.get_component_path(&arguments.locale_name);
    let name = &arguments.name;
    let r#type = &arguments.r#type;
    let locales = &arguments.locales;
    let values = &arguments.values;

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
