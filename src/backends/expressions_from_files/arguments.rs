use crate::utils::{
    aliases::SynResult,
    errors::{RequiredArgumentError, UnknownArgumentError},
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    parse::{Parse, ParseStream}, Path, Token,
    Type,
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
                "locale_name" => locale_name = Some(input.parse_string("locale_name")?),
                _ => return Err(UnknownArgumentError::new(argument)),
            };

            Ok(())
        })?;

        Ok(Self {
            locales: locales.ok_or(RequiredArgumentError::new("locales"))?,
            locales_path: locales_path.ok_or(RequiredArgumentError::new("locales_path"))?,
            expressions: expressions.ok_or(RequiredArgumentError::new("expressions"))?,
            expressions_type: expressions_type
                .ok_or(RequiredArgumentError::new("expressions_type"))?,
            path,
            locale_name: locale_name.ok_or(RequiredArgumentError::new("locale_name"))?,
        })
    }
}
