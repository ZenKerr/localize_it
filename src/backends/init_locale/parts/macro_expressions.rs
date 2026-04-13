use crate::utils::NamesProvider;
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expressions(names_provider: &NamesProvider) -> TokenStream {
    let expressions_ident = names_provider.get_hashed_name("expressions");
    let expression_path = names_provider.get_path("expression");

    quote! {
        #[macro_export]
        macro_rules! #expressions_ident {
            (
                $(
                    $name: ident $(: $content_type: ty)? => {
                        $(
                            $locale: ident: $content: expr
                        ),+ $(,)?
                    }
                ),+ $(,)?
            ) => {
                $(
                    #expression_path!($name $(: $content_type)? => {
                        $(
                            $locale: $content
                        ),+
                    });
                )+
            };
        }

        pub use #expressions_ident as expressions;
    }
}
