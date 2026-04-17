use proc_macro2::Ident;
use syn::Path;

pub struct Locale {
    pub name: Ident,
    pub path: Path,
}
