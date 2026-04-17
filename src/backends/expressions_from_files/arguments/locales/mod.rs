mod locale;

use crate::backends::expressions_from_files::arguments::locales::locale::Locale;
use derive_more::{Deref, DerefMut};
use proc_macro2::Ident;
use syn::Path;

#[derive(Deref, DerefMut)]
pub struct Locales(Vec<Locale>);

impl Locales {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, name: Ident, path: Path) {
        self.push(Locale { name, path })
    }

    pub fn enumerate_unzip(&self) -> (Vec<usize>, (Vec<&Ident>, Vec<&Path>)) {
        self.iter()
            .enumerate()
            .map(|(i, locale)| (i, (&locale.name, &locale.path)))
            .unzip()
    }
}
