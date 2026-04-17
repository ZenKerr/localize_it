use crate::{
    backends::expressions_from_files::arguments::{expression::Expression, Arguments},
    utils::NamesProvider,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn static_expression(
    expression: &Expression,
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> TokenStream {
    let locale_path = names_provider.get_path("Locale");
    let (name, r#type) = expression.decompose();
    let (locale_indexes, (locales, locale_paths)) = arguments.locales.enumerate_unzip();

    quote! {
        pub static #name: [#r#type; #locale_path::COUNT] = {
            let mut values = [#((#locale_path::#locales.to_usize(), #locale_paths::#name)),*];

            let mut i = 0;
            while i < #locale_path::COUNT {
                let mut j = 0;

                while j + 1 < #locale_path::COUNT - i {
                    if values[j].0 > values[j + 1].0 {
                        values.swap(j, j + 1);
                    }

                    j += 1;
                }

                i += 1;
            }

            [#(values[#locale_indexes].1),*]
        };
    }
}
