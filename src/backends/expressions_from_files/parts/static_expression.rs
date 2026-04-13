use crate::{
    backends::expressions_from_files::arguments::{expression::Expression, Arguments},
    utils::NamesProvider,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn static_expression(
    expression: &Expression,
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> TokenStream {
    let locale_path = names_provider.get_path("Locale");
    let name = &expression.name;
    let r#type = &expression.r#type;
    let (locales, paths) = &arguments
        .locales
        .iter()
        .map(|locale| (&locale.name, &locale.path))
        .unzip::<&Ident, &Path, Vec<&Ident>, Vec<&Path>>();
    let locale_indexes = locales
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    quote! {
        pub static #name: [#r#type; #locale_path::COUNT] = {
            let mut values = [#((#locale_path::#locales.to_usize(), #paths::#name)),*];

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
