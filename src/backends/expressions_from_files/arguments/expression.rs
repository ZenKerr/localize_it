use proc_macro2::Ident;
use syn::Type;

pub struct Expression {
    pub name: Ident,
    pub r#type: Type,
}
