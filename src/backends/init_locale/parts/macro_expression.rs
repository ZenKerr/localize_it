use crate::utils::NamesProvider;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expression(names_provider: &NamesProvider) -> TokenStream {
    let expression_ident = names_provider.get_hashed_name("expression");
    let expression_path = names_provider.get_path("expression");
    let locale_path = names_provider.get_path("Locale");

    quote! {
        #[macro_export]
        macro_rules! #expression_ident {
            (
                $name: ident => {
                    $(
                        $locale: ident: $content: expr
                    ),+ $(,)?
                }
            ) => {
                #expression_path!($name: &'static str => {$($locale: $content),+});
            };

            (
                $name: ident: $content_type: ty => {
                    $(
                        $locale: ident: $content: expr
                    ),+ $(,)?
                }
            ) => {
                pub static $name: [$content_type; #locale_path::COUNT] = {
                    let mut expression = [$($content),+];
                    let mut empty = [true; #locale_path::COUNT];

                    $(
                        let i = #locale_path::$locale.to_usize();

                        if core::mem::replace(&mut empty[i], false) {
                            expression[i] = $content;
                        } else {
                            panic!(concat!(
                                "Locale variant ",
                                stringify!($locale),
                                " is duplicated"
                            ));
                        }
                    )+

                    expression
                };
            };
        }

        pub use #expression_ident as expression;
    }
}
