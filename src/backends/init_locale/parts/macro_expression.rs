use crate::{
    backends::init_locale::arguments::Arguments,
    utils::{names::MACRO_EXPRESSION, path_argument, NamesProvider},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn macro_expression(
    arguments: &Arguments,
    names_provider: &NamesProvider,
) -> Result<TokenStream> {
    let expression_ident = names_provider.get_name(MACRO_EXPRESSION);
    let expression_hashed_ident = names_provider.get_hashed_name(MACRO_EXPRESSION);
    let expression_path = names_provider.get_path(MACRO_EXPRESSION);
    let localize_it_crate = names_provider.get_crate_name("localize_it")?;

    let path_argument = path_argument(arguments.path.clone());

    Ok(quote! {
        #[macro_export]
        macro_rules! #expression_hashed_ident {
            (
                $name: ident => {
                    $(
                        $locale: ident: $value: expr
                    ),+ $(,)?
                }
            ) => {
                #expression_path!($name: &'static str => {$($locale: $value),+});
            };

            (
                $name: ident: $r#type: ty => {
                    $(
                        $locale: ident: $value: expr
                    ),+ $(,)?
                }
            ) => {
                #localize_it_crate::__expression!(
                    name = $name,
                    r#type = $r#type,
                    locales = [$($locale),+],
                    values = [$($value),+],
                    #path_argument
                );
            };
        }

        pub use #expression_hashed_ident as #expression_ident;
    })
}
