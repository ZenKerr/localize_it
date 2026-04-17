mod variant;

use crate::backends::init_locale::arguments::variants::variant::Variant;
use derive_more::{Deref, DerefMut};
use proc_macro2::Ident;
use syn::LitStr;

#[derive(Deref, DerefMut)]
pub struct Variants(Vec<Variant>);

impl Variants {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, name: Ident, label: LitStr) {
        self.push(Variant { name, label })
    }

    pub fn unzip(&self) -> (Vec<&Ident>, Vec<&LitStr>) {
        self.iter()
            .map(|variant| (&variant.name, &variant.label))
            .unzip()
    }
}
