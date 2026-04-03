mod variant;

use crate::{
    arguments::variant::Variant,
    input::{item::Item, Input},
};
use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned, Error, Expr, ExprArray, ExprLit, ExprPath, Lit, Path};

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

        for item in input.into_iter() {
            match item {
                Item::Variant { name, label } => variants.push(Variant { name, label }),
                Item::Optional { name, value } => match name.as_str() {
                    "storage" => match value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Bool(bool),
                            ..
                        }) => storage = bool.value,
                        _ => {
                            return Err(Error::new(
                                value.span(),
                                "Expected `storage` to be a `bool`",
                            ));
                        }
                    },
                    "path" => match value {
                        Expr::Path(ExprPath { path: path_, .. }) => path = Some(path_),
                        _ => {
                            return Err(Error::new(value.span(), "Expected `path` to be a `Path`"));
                        }
                    },
                    "default" => match value {
                        Expr::Path(ExprPath { path, .. }) => {
                            if let Some(ident) = path.get_ident() {
                                default = Some(ident.clone());
                            } else {
                                return Err(Error::new(
                                    path.span(),
                                    "Expected `default` to be an `Ident`",
                                ));
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                value.span(),
                                "Expected `default` to be an `Ident`",
                            ));
                        }
                    },
                    "derive" => match value {
                        Expr::Array(ExprArray {
                            elems: elements, ..
                        }) => {
                            for element in elements {
                                match element {
                                    Expr::Path(ExprPath { path: path_, .. }) => {
                                        derive.push(path_.clone())
                                    }
                                    _ => {
                                        return Err(Error::new(
                                            element.span(),
                                            "Expected `derive` to be an array of `Path`",
                                        ));
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(Error::new(
                                value.span(),
                                "Expected `derive` to be an array of `Path`",
                            ));
                        }
                    },
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
