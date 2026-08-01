use crate::utils::aliases::{LocalizeItError, LocalizeItResult};
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Ident, LineColumn, Span};
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::{Path, PathSegment};

pub struct NamesProvider {
    hash: u64,
    path: Option<Path>,
}

impl NamesProvider {
    pub fn new(path: Option<Path>) -> Self {
        let span = Span::call_site();
        let file = span.file();
        let LineColumn { line, column } = span.start();

        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        line.hash(&mut hasher);
        column.hash(&mut hasher);
        let hash = hasher.finish();

        Self { hash, path }
    }

    pub fn get_name(name: &str) -> Ident {
        Ident::new(name, Span::call_site())
    }

    pub fn get_path(path: &str) -> LocalizeItResult<Path> {
        syn::parse_str(path).map_err(Into::into)
    }

    pub fn get_hashed_name(&self, name: &str) -> Ident {
        Ident::new(&format!("__{}_{}", name, self.hash), Span::call_site())
    }

    pub fn get_component_path(&self, name: &str) -> Path {
        let name = Self::get_name(name);

        self.path
            .clone()
            .map(|mut path| {
                path.segments.push(PathSegment::from(name.clone()));

                path
            })
            .unwrap_or(Path::from(name))
    }

    pub fn get_crate_name(name: &str) -> LocalizeItResult<Ident> {
        let found_crate = crate_name(name).map_err(LocalizeItError::on_crate_not_found(name))?;
        let crate_name = match found_crate {
            FoundCrate::Itself => "crate".to_string(),
            FoundCrate::Name(name) => name,
        };

        Ok(Ident::new(&crate_name, Span::call_site()))
    }
}
