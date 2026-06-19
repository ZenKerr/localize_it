use crate::utils::{
    aliases::SynResult,
    errors::{LocaleVariantPositionError, NoLocaleVariantError, UnknownArgumentError},
    names::DEFAULT_ENUM_LOCALE,
    typed_parse::TypedParse,
};
use proc_macro2::Ident;
use syn::{
    LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

pub struct Arguments {
    pub variants: Vec<Ident>,
    pub variants_label: Vec<LitStr>,
    pub storage: bool,
    pub path: Option<Path>,
    pub default: Ident,
    pub locale_name: String,
    pub derive: Vec<Path>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut variants = Vec::new();
        let mut variants_label = Vec::new();
        let mut storage = false;
        let mut path = None;
        let mut default = None;
        let mut locale_name = None;
        let mut derive = Vec::new();

        let mut variants_is_end = false;
        input.parse_arguments(|argument, processor| {
            if input.peek(Token![=]) && !input.peek(Token![=>]) {
                input.parse::<Token![=]>()?;

                match processor.process(&argument)?.as_str() {
                    "storage" => storage = input.parse_bool("storage")?,
                    "path" => path = Some(input.parse_path("path")?),
                    "default" => default = Some(input.parse_ident("default")?),
                    "locale_name" => locale_name = Some(input.parse_ident("locale_name")?),
                    "derive" => derive = input.parse_array("derive", Path::parse)?,
                    _ => Err(UnknownArgumentError::new(argument))?,
                }

                variants_is_end = true;
            } else if variants_is_end {
                Err(LocaleVariantPositionError::new(argument))?;
            } else {
                let label = if input.peek(Token![=>]) {
                    input.parse::<Token![=>]>()?;

                    input.parse()?
                } else {
                    LitStr::new(&argument.to_string(), argument.span())
                };

                variants.push(argument);
                variants_label.push(label);
            }

            Ok(())
        })?;

        let default =
            default.unwrap_or(variants.first().ok_or(NoLocaleVariantError::new())?.clone());
        let locale_name = locale_name.map_or(DEFAULT_ENUM_LOCALE.to_string(), |locale_name| {
            locale_name.to_string()
        });

        Ok(Self {
            variants,
            variants_label,
            storage,
            path,
            default,
            locale_name,
            derive,
        })
    }
}
