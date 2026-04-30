use crate::utils::{
    names::{MACRO_EXPRESSION, MACRO_EXPRESSIONS},
    NamesProvider,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expressions(names_provider: &NamesProvider) -> TokenStream {
    let expressions_ident = names_provider.get_name(MACRO_EXPRESSIONS);
    let expressions_hashed_ident = names_provider.get_hashed_name(MACRO_EXPRESSIONS);
    let expression_path = names_provider.get_path(MACRO_EXPRESSION);

    quote! {
        #[macro_export]
        macro_rules! #expressions_hashed_ident {
            (
                $(
                    $name: ident $(: $r#type: ty)? => {
                        $(
                            $locale: ident: $value: expr
                        ),+ $(,)?
                    }
                ),+ $(,)?
            ) => {
                $(
                    #expression_path!($name $(: $r#type)? => {
                        $(
                            $locale: $value
                        ),+
                    });
                )+
            };
        }

        pub use #expressions_hashed_ident as #expressions_ident;
    }
}
