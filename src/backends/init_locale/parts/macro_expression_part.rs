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
                ($name: ident => $content: expr) => {
                    #expression_part_path!($name: &'static str => $content);
                };

                ($name: ident: $content_type: ty => $content: expr) => {
                    pub static $name: $content_type = $content;
                };
            }

            pub use #expression_part_hashed_ident as #expression_part_ident;
        }
    } else {
        TokenStream::new()
    }
}
