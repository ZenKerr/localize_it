use crate::utils::{
    errors::{required_argument_error, unknown_argument_error},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream}, Path, Result, Token,
    Type,
};

pub struct Arguments {
    pub locales: Vec<Ident>,
    pub locales_path: Vec<Path>,
    pub expressions: Vec<Ident>,
    pub expressions_type: Vec<Type>,
    pub path: Option<Path>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut locales = None;
        let mut locales_path = None;
        let mut expressions = None;
        let mut expressions_type = None;
        let mut path = None;

        input.parse_arguments(|argument, processor| {
            input.parse::<Token![=]>()?;

            match processor.process(&argument)?.as_str() {
                "locales" => locales = Some(input.parse_array("locales", Ident::parse)?),
                "locales_path" => {
                    locales_path = Some(input.parse_array("locales_path", Path::parse)?)
                }
                "expressions" => {
                    expressions = Some(input.parse_array("expressions", Ident::parse)?)
                }
                "expressions_type" => {
                    expressions_type = Some(input.parse_array("expressions_type", Type::parse)?)
                }
                "path" => path = Some(input.parse_path("path")?),
                _ => return Err(unknown_argument_error(argument)),
            };

            Ok(())
        })?;

        Ok(Self {
            locales: locales.ok_or(required_argument_error("locales"))?,
            locales_path: locales_path.ok_or(required_argument_error("locales_path"))?,
            expressions: expressions.ok_or(required_argument_error("expressions"))?,
            expressions_type: expressions_type
                .ok_or(required_argument_error("expressions_type"))?,
            path,
        })
    }
}
