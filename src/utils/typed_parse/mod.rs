mod for_parse_stream;

use crate::utils::ArgumentProcessor;
use proc_macro2::Ident;
use syn::{parse::ParseStream, Path, Result, Type};

pub trait TypedParse {
    fn parse_bool(self, name: &str) -> Result<bool>;

    fn parse_ident(self, name: &str) -> Result<Ident>;

    fn parse_type(self, name: &str) -> Result<Type>;

    fn parse_path(self, name: &str) -> Result<Path>;

    fn parse_array<T>(
        self,
        name: &str,
        parse_function: fn(ParseStream) -> Result<T>,
    ) -> Result<Vec<T>>;

    fn parse_arguments<T>(self, parse_function: T) -> Result<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> Result<()>;
}
