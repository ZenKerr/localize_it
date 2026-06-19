use crate::utils::{
    aliases::SynResult,
    errors::{RequiredArgumentError, UnknownArgumentError},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    Expr, Path, Token, Type,
    parse::{Parse, ParseStream},
};

pub struct Arguments {
    pub name: Ident,
    pub r#type: Type,
    pub locales: Vec<Ident>,
    pub values: Vec<Expr>,
    pub path: Option<Path>,
    pub locale_name: String,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut name = None;
        let mut r#type = None;
        let mut locales = None;
        let mut values = None;
        let mut path = None;
        let mut locale_name = None;

        input.parse_arguments(|argument, processor| {
            input.parse::<Token![=]>()?;

            match processor.process(&argument)?.as_str() {
                "name" => name = Some(input.parse_ident("name")?),
                "r#type" => r#type = Some(input.parse_type("r#type")?),
                "locales" => locales = Some(input.parse_array("locales", Ident::parse)?),
                "values" => values = Some(input.parse_array("values", Expr::parse)?),
                "path" => path = Some(input.parse_path("path")?),
                "locale_name" => locale_name = Some(input.parse_string("locale_name")?),
                _ => Err(UnknownArgumentError::new(argument))?,
            };

            Ok(())
        })?;

        Ok(Self {
            name: name.ok_or(RequiredArgumentError::new("name"))?,
            r#type: r#type.ok_or(RequiredArgumentError::new("r#type"))?,
            locales: locales.ok_or(RequiredArgumentError::new("locales"))?,
            values: values.ok_or(RequiredArgumentError::new("values"))?,
            path,
            locale_name: locale_name.ok_or(RequiredArgumentError::new("locale_name"))?,
        })
    }
}
