use proc_macro2::{Ident, LineColumn, Span};
use proc_macro_crate::{crate_name, FoundCrate};
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::{Error, Path, PathSegment};

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

    pub fn get_name(&self, name: &str) -> Ident {
        Ident::new(name, Span::call_site())
    }

    pub fn get_hashed_name(&self, name: &str) -> Ident {
        Ident::new(&format!("__{}_{}", name, self.hash), Span::call_site())
    }

    pub fn get_path(&self, name: &str) -> Path {
        let name = self.get_name(name);

        self.path
            .clone()
            .map(|mut path| {
                path.segments.push(PathSegment::from(name.clone()));

                path
            })
            .unwrap_or(Path::from(name))
    }

    pub fn get_crate_name(&self, name: &str) -> Result<Ident, Error> {
        let span = Span::call_site();

        let found_crate =
            crate_name(name).map_err(|_| Error::new(span, format!("Crate `{name}` not found")))?;

        Ok(match found_crate {
            FoundCrate::Itself => Ident::new("crate", span),
            FoundCrate::Name(name) => Ident::new(&name, span),
        })
    }
}
