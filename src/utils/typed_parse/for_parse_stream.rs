use crate::utils::{
    ArgumentProcessor,
    aliases::SynResult,
    errors::{NoCommaBetweenArgumentError, TypeError},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{LitBool, LitStr, Path, Token, Type, bracketed, parse::ParseStream};

impl TypedParse for ParseStream<'_> {
    fn parse_bool(self, name: &str) -> SynResult<bool> {
        Ok(self
            .parse::<LitBool>()
            .map_err(TypeError::map(name, "bool"))?
            .value)
    }

    fn parse_string(self, name: &str) -> SynResult<String> {
        Ok(self
            .parse::<LitStr>()
            .map_err(TypeError::map(name, "String"))?
            .value())
    }

    fn parse_ident(self, name: &str) -> SynResult<Ident> {
        self.parse().map_err(TypeError::map(name, "Ident"))
    }

    fn parse_type(self, name: &str) -> SynResult<Type> {
        self.parse().map_err(TypeError::map(name, "Type"))
    }

    fn parse_path(self, name: &str) -> SynResult<Path> {
        self.parse().map_err(TypeError::map(name, "Path"))
    }

    fn parse_array<T>(
        self,
        name: &str,
        parse_function: fn(ParseStream) -> SynResult<T>,
    ) -> SynResult<Vec<T>> {
        let content;
        bracketed!(content in self);

        Ok(content
            .parse_terminated(parse_function, Token![,])
            .map_err(TypeError::map(name, "Array"))?
            .into_iter()
            .collect())
    }

    fn parse_arguments<T>(self, mut parse_function: T) -> SynResult<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> SynResult<()>,
    {
        let mut argument_processor = ArgumentProcessor::new();

        while !self.is_empty() {
            let argument_name = self.parse::<Ident>()?;

            parse_function(argument_name, &mut argument_processor)?;

            if self.peek(Token![,]) {
                self.parse::<Token![,]>()?;
            } else if !self.is_empty() {
                Err(NoCommaBetweenArgumentError::new(self))?
            }
        }

        Ok(())
    }
}
