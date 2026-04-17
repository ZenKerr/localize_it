pub mod expression;
mod locales;

use crate::{
    backends::expressions_from_files::arguments::{expression::Expression, locales::Locales},
    utils::{Argument, Input},
};
use proc_macro2::Span;
use syn::{parse_str, spanned::Spanned, Error, Path};

pub struct Arguments {
    pub locales: Locales,
    pub expressions: Vec<Expression>,
    pub path: Option<Path>,
}

impl Arguments {
    pub fn new(input: Input) -> Result<Self, Error> {
        let mut locales = Locales::new();
        let mut expressions = Vec::new();
        let mut path = None;

        for argument in input.into_iter() {
            match argument {
                Argument::Mapped { .. } => {
                    return Err(Error::new(
                        Span::call_site(),
                        "Expected only named arguments",
                    ));
                }
                Argument::Named { name, value } => match name.as_str() {
                    "locales" => {
                        for locale in Input::parse_array("locales", &value, Input::parse_tuple)? {
                            let name = Input::parse_ident(
                                "locale name",
                                locale
                                    .get(0)
                                    .ok_or(Error::new(value.span(), "Expected locale name"))?,
                            )?;
                            let path = Input::parse_path(
                                "locale path",
                                locale
                                    .get(1)
                                    .ok_or(Error::new(value.span(), "Expected locale path"))?,
                            )?;

                            locales.add(name, path);
                        }
                    }
                    "expressions" => {
                        for expression in
                            Input::parse_array("expressions", &value, Input::parse_tuple)?
                        {
                            let name = Input::parse_ident(
                                "expression name",
                                expression
                                    .get(0)
                                    .ok_or(Error::new(value.span(), "Expected expression name"))?,
                            )?;
                            let r#type = Input::parse_optional(
                                "expression type",
                                expression.get(1),
                                parse_str("&'static str")?,
                                Input::parse_type,
                            )?;

                            expressions.push(Expression { name, r#type });
                        }
                    }
                    "path" => path = Some(Input::parse_path("path", &value)?),
                    _ => {
                        return Err(Error::new(
                            value.span(),
                            format!("Unknown parameter `{name}`"),
                        ));
                    }
                },
            }
        }

        Ok(Self {
            locales,
            expressions,
            path,
        })
    }
}
