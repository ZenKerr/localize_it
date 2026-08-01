use crate::utils::{
    ArgumentProcessor,
    aliases::{LocalizeItError, LocalizeItResult},
    bracketed,
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    LitBool, LitStr, Path, Token, Type,
    parse::{Parse, ParseStream},
};

impl TypedParse for ParseStream<'_> {
    fn parse_bool(self, name: &str) -> LocalizeItResult<bool> {
        Ok(self
            .parse::<LitBool>()
            .map_err(LocalizeItError::on_type_error(name, "bool"))?
            .value)
    }

    fn parse_string(self, name: &str) -> LocalizeItResult<String> {
        Ok(self
            .parse::<LitStr>()
            .map_err(LocalizeItError::on_type_error(name, "String"))?
            .value())
    }

    fn parse_ident(self, name: &str) -> LocalizeItResult<Ident> {
        self.parse()
            .map_err(LocalizeItError::on_type_error(name, "Ident"))
    }

    fn parse_type(self, name: &str) -> LocalizeItResult<Type> {
        self.parse()
            .map_err(LocalizeItError::on_type_error(name, "Type"))
    }

    fn parse_path(self, name: &str) -> LocalizeItResult<Path> {
        self.parse()
            .map_err(LocalizeItError::on_type_error(name, "Path"))
    }

    fn parse_array<T>(self, name: &str) -> LocalizeItResult<Vec<T>>
    where
        T: Parse,
    {
        Ok(bracketed(self)?
            .parse_terminated(T::parse, Token![,])
            .map_err(LocalizeItError::on_type_error(name, "Array"))?
            .into_iter()
            .collect())
    }

    fn parse_arguments<T>(self, mut parse_function: T) -> LocalizeItResult<()>
    where
        T: FnMut(Ident, &mut ArgumentProcessor) -> LocalizeItResult<()>,
    {
        let mut argument_processor = ArgumentProcessor::new();

        while !self.is_empty() {
            let argument_name = self.parse::<Ident>()?;

            parse_function(argument_name, &mut argument_processor)?;

            if self.peek(Token![,]) {
                self.parse::<Token![,]>()?;
            } else if !self.is_empty() {
                Err(LocalizeItError::NoCommaBetweenArgument(self.span()))?;
            }
        }

        Ok(())
    }
}
