use crate::utils::{errors::type_error, typed_parse::TypedParse, ArgumentProcessor};
use proc_macro2::Ident;
use syn::{bracketed, parse::ParseStream, LitBool, Path, Result, Token, Type};

impl TypedParse for ParseStream<'_> {
    fn parse_bool(self, name: &str) -> Result<bool> {
        Ok(self
            .parse::<LitBool>()
            .map_err(type_error(name, "bool"))?
            .value)
    }

    fn parse_ident(self, name: &str) -> Result<Ident> {
        self.parse().map_err(type_error(name, "Ident"))
    }

    fn parse_type(self, name: &str) -> Result<Type> {
        self.parse().map_err(type_error(name, "Type"))
    }

    fn parse_path(self, name: &str) -> Result<Path> {
        self.parse().map_err(type_error(name, "Path"))
    }

    fn parse_array<T>(
        self,
        name: &str,
        parse_function: fn(ParseStream) -> Result<T>,
    ) -> Result<Vec<T>> {
        let content;
        bracketed!(content in self);

        Ok(content
            .parse_terminated(parse_function, Token![,])
            .map_err(type_error(name, "Array"))?
            .into_iter()
            .collect())
    }

    fn parse_arguments<T>(self, mut parse_function: T) -> Result<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> Result<()>,
    {
        let mut argument_processor = ArgumentProcessor::new();

        while !self.is_empty() {
            let argument_name = self.parse::<Ident>()?;

            parse_function(argument_name, &mut argument_processor)?;

            if self.peek(Token![,]) {
                self.parse::<Token![,]>()?;
            }
        }

        Ok(())
    }
}
