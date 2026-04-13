mod variant;

use crate::{
    backends::init_locale::arguments::variant::Variant,
    utils::{Argument, Input},
};
use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Error, Path};

pub struct Arguments {
    pub variants: Vec<Variant>,
    pub storage: bool,
    pub path: Option<Path>,
    pub default: Ident,
    pub derive: Vec<Path>,
}

impl Arguments {
    pub fn new(input: Input) -> Result<Self, Error> {
        let mut variants = Vec::new();
        let mut storage = false;
        let mut path = None;
        let mut default = None;
        let mut derive = Vec::new();

        for argument in input.into_iter() {
            match argument {
                Argument::Mapped { name, label } => variants.push(Variant { name, label }),
                Argument::Named { name, value } => match name.as_str() {
                    "storage" => storage = Input::parse_bool("storage", &value)?,
                    "path" => path = Some(Input::parse_path("path", &value)?),
                    "default" => default = Some(Input::parse_ident("default", &value)?),
                    "derive" => derive = Input::parse_array("derive", &value, Input::parse_path)?,
                    _ => {
                        return Err(Error::new(
                            value.span(),
                            format!("Unknown parameter `{name}`"),
                        ));
                    }
                },
            }
        }

        let default = default.unwrap_or(
            variants
                .first()
                .ok_or(Error::new(
                    Span::call_site(),
                    "Expected at least one locale variant",
                ))?
                .name
                .clone(),
        );

        Ok(Self {
            variants,
            storage,
            path,
            default,
            derive,
        })
    }
}
