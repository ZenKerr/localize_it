mod for_parse_stream;

use crate::utils::{aliases::SynResult, ArgumentProcessor};
use proc_macro2::Ident;
use syn::{parse::ParseStream, Path, Type};

pub trait TypedParse {
    fn parse_bool(self, name: &str) -> SynResult<bool>;

    fn parse_string(self, name: &str) -> SynResult<String>;

    fn parse_ident(self, name: &str) -> SynResult<Ident>;

    fn parse_type(self, name: &str) -> SynResult<Type>;

    fn parse_path(self, name: &str) -> SynResult<Path>;

    fn parse_array<T>(
        self,
        name: &str,
        parse_function: fn(ParseStream) -> SynResult<T>,
    ) -> SynResult<Vec<T>>;

    fn parse_arguments<T>(self, parse_function: T) -> SynResult<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> SynResult<()>;
}
