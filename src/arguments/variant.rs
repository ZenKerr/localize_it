use proc_macro2::Ident;
use syn::LitStr;

pub struct Variant {
    pub name: Ident,
    pub label: LitStr,
}
