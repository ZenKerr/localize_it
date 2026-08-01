use crate::utils::{
    aliases::{LocalizeItError, SynResult},
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
                "locales" => locales = Some(input.parse_array("locales")?),
                "values" => values = Some(input.parse_array("values")?),
                "path" => path = Some(input.parse_path("path")?),
                "locale_name" => locale_name = Some(input.parse_string("locale_name")?),
                _ => Err(LocalizeItError::UnknownArgument(argument))?,
            }

            Ok(())
        })?;

        Ok(Self {
            name: name.ok_or(LocalizeItError::RequiredArgument("name"))?,
            r#type: r#type.ok_or(LocalizeItError::RequiredArgument("r#type"))?,
            locales: locales.ok_or(LocalizeItError::RequiredArgument("locales"))?,
            values: values.ok_or(LocalizeItError::RequiredArgument("values"))?,
            path,
            locale_name: locale_name.ok_or(LocalizeItError::RequiredArgument("locale_name"))?,
        })
    }
}
