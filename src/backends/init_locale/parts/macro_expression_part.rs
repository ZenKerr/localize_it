use crate::utils::{names::MACRO_EXPRESSION_PART, NamesProvider};
use proc_macro2::TokenStream;
use quote::quote;

pub fn macro_expression_part(names_provider: &NamesProvider) -> TokenStream {
    if cfg!(feature = "from_files") {
        let expression_part_ident = names_provider.get_name(MACRO_EXPRESSION_PART);
        let expression_part_hashed_ident = names_provider.get_hashed_name(MACRO_EXPRESSION_PART);
        let expression_part_path = names_provider.get_path(MACRO_EXPRESSION_PART);

        quote! {
            #[macro_export]
            macro_rules! #expression_part_hashed_ident {
                ($name: ident => $value: expr) => {
                    #expression_part_path!($name: &'static str => $value);
                };

                ($name: ident: $r#type: ty => $value: expr) => {
                    pub static $name: $r#type = $value;
                };
            }

            pub use #expression_part_hashed_ident as #expression_part_ident;
        }
    } else {
        TokenStream::new()
    }
}
