pub mod argument;
mod types;

use crate::utils::input::types::ParseFunction;
use argument::Argument;
use proc_macro2::Ident;
use quote::ToTokens;
use std::{collections::HashSet, vec::IntoIter};
use syn::{
    parse::{Parse, ParseStream}, parse2, spanned::Spanned, Error, Expr, ExprArray, ExprLit, ExprPath, ExprTuple, Lit,
    Path,
    Token,
    Type,
};

pub struct Input {
    arguments: Vec<Argument>,
}

impl Input {
    pub fn parse_bool(name: &str, value: &Expr) -> Result<bool, Error> {
        match value {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(bool),
                ..
            }) => Ok(bool.value),
            _ => Err(Error::new(
                value.span(),
                format!("Expected `{name}` to be a `bool`"),
            )),
        }
    }

    pub fn parse_ident(name: &str, value: &Expr) -> Result<Ident, Error> {
        match value {
            Expr::Path(ExprPath { path, .. }) => {
                if let Some(ident) = path.get_ident() {
                    Ok(ident.clone())
                } else {
                    Err(Error::new(
                        path.span(),
                        format!("Expected `{name}` to be an `Ident`"),
                    ))
                }
            }
            _ => Err(Error::new(
                value.span(),
                format!("Expected `{name}` to be an `Ident`"),
            )),
        }
    }

    pub fn parse_type(name: &str, value: &Expr) -> Result<Type, Error> {
        let r#type = parse2(value.to_token_stream());

        r#type.map_err(|_| Error::new(value.span(), format!("Expected `{name}` to be a `Type`")))
    }

    pub fn parse_path(name: &str, value: &Expr) -> Result<Path, Error> {
        match value {
            Expr::Path(ExprPath { path, .. }) => Ok(path.clone()),
            Expr::Group(group) => Self::parse_path(name, &*group.expr),
            _ => Err(Error::new(
                value.span(),
                format!("Expected `{name}` to be a `Path`"),
            )),
        }
    }

    pub fn parse_tuple(name: &str, value: &Expr) -> Result<Vec<Expr>, Error> {
        match value {
            Expr::Tuple(ExprTuple {
                elems: elements, ..
            }) => Ok(elements.into_iter().map(|value| value.clone()).collect()),
            _ => Err(Error::new(
                value.span(),
                format!("Expected `{name}` to be an `Tuple`"),
            )),
        }
    }

    pub fn parse_array<T>(
        name: &str,
        value: &Expr,
        parse_function: ParseFunction<T>,
    ) -> Result<Vec<T>, Error> {
        match value {
            Expr::Array(ExprArray {
                elems: elements, ..
            }) => elements
                .into_iter()
                .map(|element| parse_function(&format!("{} item", name), element))
                .collect(),
            _ => Err(Error::new(
                value.span(),
                format!("Expected `{name}` to be an array"),
            )),
        }
    }

    pub fn parse_optional<T>(
        name: &str,
        value: Option<&Expr>,
        default: T,
        parse_function: ParseFunction<T>,
    ) -> Result<T, Error> {
        match value {
            Some(value) => parse_function(name, value),
            None => Ok(default),
        }
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut arguments = Vec::new();

        let mut exists_names = HashSet::new();
        while !input.is_empty() {
            let argument = input.parse()?;

            match &argument {
                Argument::Mapped { name, .. } => {
                    if exists_names.len() > 0 {
                        return Err(Error::new(
                            name.span(),
                            "Expected variants list to precede optional arguments",
                        ));
                    }
                }
                Argument::Named { name, .. } => {
                    if !exists_names.insert(name.clone()) {
                        return Err(Error::new(
                            name.span(),
                            format!("Duplicate argument `{name}`"),
                        ));
                    }
                }
            }

            arguments.push(argument);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { arguments })
    }
}

impl IntoIterator for Input {
    type Item = Argument;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.arguments.into_iter()
    }
}
