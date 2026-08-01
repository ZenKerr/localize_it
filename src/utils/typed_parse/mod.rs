mod for_parse_stream;

use crate::utils::{ArgumentProcessor, aliases::LocalizeItResult};
use proc_macro2::Ident;
use syn::{Path, Type, parse::Parse};

pub trait TypedParse {
    fn parse_bool(self, name: &str) -> LocalizeItResult<bool>;

    fn parse_string(self, name: &str) -> LocalizeItResult<String>;

    fn parse_ident(self, name: &str) -> LocalizeItResult<Ident>;

    fn parse_type(self, name: &str) -> LocalizeItResult<Type>;

    fn parse_path(self, name: &str) -> LocalizeItResult<Path>;

    fn parse_array<T>(self, name: &str) -> LocalizeItResult<Vec<T>>
    where
        T: Parse;

    fn parse_arguments<T>(self, parse_function: T) -> LocalizeItResult<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> LocalizeItResult<()>;
}
