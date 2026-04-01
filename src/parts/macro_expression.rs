use crate::names::Names;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expression(names: &Names) -> TokenStream {
    let expression_ident = names.get_hashed_name("expression");
    let expression_path = names.get_path("expression");
    let locale_path = names.get_path("Locale");

    quote! {
        #[macro_export]
        macro_rules! #expression_ident {
            (
                $name: ident => {
                    $(
                        $lang: ident: $expression: expr
                    ),+ $(,)?
                }
            ) => {
                #expression_path!($name: &'static str => {$($lang: $expression),+});
            };

            (
                $name: ident: $content_type: ty => {
                    $(
                        $lang: ident: $content: expr
                    ),+ $(,)?
                }
            ) => {
                pub static $name: [$content_type; #locale_path::COUNT] = {
                    let mut expression = [$($content),+];
                    let mut empty = [true; #locale_path::COUNT];

                    $(
                        let i = #locale_path::$lang.to_usize();

                        if core::mem::replace(&mut empty[i], false) {
                            expression[i] = $content;
                        } else {
                            panic!(concat!(
                                "Locale variant ",
                                stringify!($lang),
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
