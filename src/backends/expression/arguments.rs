use crate::utils::{
    errors::{required_argument_error, unknown_argument_error},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream}, Expr, Path, Result, Token,
    Type,
};

pub struct Arguments {
    pub name: Ident,
    pub r#type: Type,
    pub locales: Vec<Ident>,
    pub values: Vec<Expr>,
    pub path: Option<Path>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut name = None;
        let mut r#type = None;
        let mut locales = None;
        let mut values = None;
        let mut path = None;

        input.parse_arguments(|argument, processor| {
            input.parse::<Token![=]>()?;

            match processor.process(&argument)?.as_str() {
                "name" => name = Some(input.parse_ident("name")?),
                "r#type" => r#type = Some(input.parse_type("r#type")?),
                "locales" => locales = Some(input.parse_array("locales", Ident::parse)?),
                "values" => values = Some(input.parse_array("values", Expr::parse)?),
                "path" => path = Some(input.parse_path("path")?),
                _ => return Err(unknown_argument_error(argument)),
            };

            Ok(())
        })?;

        Ok(Self {
            name: name.ok_or(required_argument_error("name"))?,
            r#type: r#type.ok_or(required_argument_error("r#type"))?,
            locales: locales.ok_or(required_argument_error("locales"))?,
            values: values.ok_or(required_argument_error("values"))?,
            path,
        })
    }
}
