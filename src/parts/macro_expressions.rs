use crate::names::Names;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expressions(names: &Names) -> TokenStream {
    let expressions_ident = names.get_hashed_name("expressions");
    let expression_path = names.get_path("expression");

    quote! {
        #[macro_export]
        macro_rules! #expressions_ident {
            (
                $(
                    $name: ident $(: $content_type: ty)? => {
                        $(
                            $lang: ident: $content: expr
                        ),+ $(,)?
                    }
                ),+ $(,)?
            ) => {
                $(
                    #expression_path!($name $(: $content_type)? => {
                        $(
                            $lang: $content
                        ),+
                    });
                )+
            };
        }

        pub use #expressions_ident as expressions;
    }
}
