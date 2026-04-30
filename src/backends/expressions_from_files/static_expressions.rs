use crate::{
    backends::expressions_from_files::arguments::Arguments,
    utils::{path_argument, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn static_expressions(
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> Result<TokenStream> {
    let localize_it_crate = names_provider.get_crate_name("localize_it")?;
    let locales = &arguments.locales;
    let locales_path = &arguments.locales_path;

    let path_argument = path_argument(arguments.path.clone());

    let expressions = arguments
        .expressions
        .iter()
        .zip(&arguments.expressions_type)
        .map(|(name, r#type)| {
            quote! {
                #localize_it_crate::__expression!(
                    name = #name,
                    r#type = #r#type,
                    locales = [#(#locales),*],
                    values = [#(#locales_path::#name),*],
                    #path_argument
                );
            }
        });

    Ok(quote! {
        #(#expressions)*
    })
}
