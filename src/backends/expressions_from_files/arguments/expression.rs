use proc_macro2::Ident;
use syn::Type;

pub struct Expression {
    pub name: Ident,
    pub r#type: Type,
}

impl Expression {
    pub fn decompose(&self) -> (&Ident, &Type) {
        (&self.name, &self.r#type)
    }
}
