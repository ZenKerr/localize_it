use crate::utils::{
    NamesProvider,
    names::{MACRO_EXPRESSION_PART, MACRO_EXPRESSION_PARTS},
};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expression_parts(names_provider: &NamesProvider) -> TokenStream {
    if cfg!(feature = "from_files") {
        let expression_parts_ident = NamesProvider::get_name(MACRO_EXPRESSION_PARTS);
        let expression_parts_hashed_ident = names_provider.get_hashed_name(MACRO_EXPRESSION_PARTS);
        let expression_part_path = names_provider.get_component_path(MACRO_EXPRESSION_PART);

        quote! {
            #[macro_export]
            macro_rules! #expression_parts_hashed_ident {
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

            pub use #expression_parts_hashed_ident as #expression_parts_ident;
        }
    } else {
        TokenStream::new()
    }
}
