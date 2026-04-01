use crate::arguments::Arguments;
use proc_macro2::{Ident, LineColumn, Span};
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::{Path, PathSegment};

pub struct Names<'a> {
    hash: u64,
    arguments: &'a Arguments,
}

impl<'a> Names<'a> {
    pub fn new(arguments: &'a Arguments) -> Self {
        let span = Span::call_site();
        let file = span.file();
        let LineColumn { line, column } = span.start();

        let mut hasher = DefaultHasher::new();
        file.hash(&mut hasher);
        line.hash(&mut hasher);
        column.hash(&mut hasher);
        let hash = hasher.finish();

        Self { hash, arguments }
    }

    pub fn get_name(&self, name: &str) -> Ident {
        Ident::new(name, Span::call_site())
    }

    pub fn get_hashed_name(&self, name: &str) -> Ident {
        Ident::new(&format!("__{}_{}", name, self.hash), Span::call_site())
    }

    pub fn get_path(&self, name: &str) -> Path {
        let name = self.get_name(name);

        self.arguments
            .path
            .clone()
            .map(|mut path| {
                path.segments.push(PathSegment::from(name.clone()));

                path
            })
            .unwrap_or(Path::from(name))
    }
}
