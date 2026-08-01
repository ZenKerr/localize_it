use crate::utils::{
    aliases::{LocalizeItError, SynResult},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    Path, Token, Type,
    parse::{Parse, ParseStream},
};

pub struct Arguments {
    pub locales: Vec<Ident>,
    pub locales_path: Vec<Path>,
    pub expressions: Vec<Ident>,
    pub expressions_type: Vec<Type>,
    pub path: Option<Path>,
    pub locale_name: String,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut locales = None;
        let mut locales_path = None;
        let mut expressions = None;
        let mut expressions_type = None;
        let mut path = None;
        let mut locale_name = None;

        input.parse_arguments(|argument, processor| {
            input.parse::<Token![=]>()?;

            match processor.process(&argument)?.as_str() {
                "locales" => locales = Some(input.parse_array("locales")?),
                "locales_path" => locales_path = Some(input.parse_array("locales_path")?),
                "expressions" => expressions = Some(input.parse_array("expressions")?),
                "expressions_type" => {
                    expressions_type = Some(input.parse_array("expressions_type")?)
                }
                "path" => path = Some(input.parse_path("path")?),
                "locale_name" => locale_name = Some(input.parse_string("locale_name")?),
                _ => Err(LocalizeItError::UnknownArgument(argument))?,
            };

            Ok(())
        })?;

        Ok(Self {
            locales: locales.ok_or(LocalizeItError::RequiredArgument("locales"))?,
            locales_path: locales_path.ok_or(LocalizeItError::RequiredArgument("locales_path"))?,
            expressions: expressions.ok_or(LocalizeItError::RequiredArgument("expressions"))?,
            expressions_type: expressions_type
                .ok_or(LocalizeItError::RequiredArgument("expressions_type"))?,
            path,
            locale_name: locale_name.ok_or(LocalizeItError::RequiredArgument("locale_name"))?,
        })
    }
}
