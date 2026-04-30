use crate::utils::{errors::unknown_argument_error, typed_parse::TypedParse};
use proc_macro2::{Ident, Span};
use syn::{
    parse::{Parse, ParseStream}, Error, LitStr, Path, Result,
    Token,
};

pub struct Arguments {
    pub variants: Vec<Ident>,
    pub variants_label: Vec<LitStr>,
    pub storage: bool,
    pub path: Option<Path>,
    pub default: Ident,
    pub derive: Vec<Path>,
}

impl Parse for Arguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut variants = Vec::new();
        let mut variants_label = Vec::new();
        let mut storage = false;
        let mut path = None;
        let mut default = None;
        let mut derive = Vec::new();

        let mut variants_is_end = false;
        input.parse_arguments(|argument, processor| {
            if input.peek(Token![=]) && !input.peek(Token![=>]) {
                input.parse::<Token![=]>()?;

                match processor.process(&argument)?.as_str() {
                    "storage" => storage = input.parse_bool("storage")?,
                    "path" => path = Some(input.parse_path("path")?),
                    "default" => default = Some(input.parse_ident("default")?),
                    "derive" => derive = input.parse_array("derive", Path::parse)?,
                    _ => return Err(unknown_argument_error(argument)),
                }

                variants_is_end = true;
            } else if variants_is_end {
                return Err(Error::new(
                    argument.span(),
                    "Expected variants list to precede optional arguments",
                ));
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

        let default = default.unwrap_or(
            variants
                .first()
                .ok_or(Error::new(
                    Span::call_site(),
                    "Expected at least one locale variant",
                ))?
                .clone(),
        );

        Ok(Self {
            variants,
            variants_label,
            storage,
            path,
            default,
            derive,
        })
    }
}
