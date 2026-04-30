use crate::utils::{
    names::{MACRO_EXPRESSIONS_PART, MACRO_EXPRESSION_PART},
    NamesProvider,
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expressions_part(names_provider: &NamesProvider) -> TokenStream {
    if cfg!(feature = "from_files") {
        let expressions_part_ident = names_provider.get_name(MACRO_EXPRESSIONS_PART);
        let expressions_part_hashed_ident = names_provider.get_hashed_name(MACRO_EXPRESSIONS_PART);
        let expression_part_path = names_provider.get_path(MACRO_EXPRESSION_PART);

        quote! {
            #[macro_export]
            macro_rules! #expressions_part_hashed_ident {
                (
                    $(
                        $name: ident $(: $r#type: ty)? => $value: expr
                    ),+ $(,)?
                ) => {
                    $(
                        #expression_part_path!($name $(: $r#type)? => $value);
                    )+
                }
            }

            pub use #expressions_part_hashed_ident as #expressions_part_ident;
        }
    } else {
        TokenStream::new()
    }
}
