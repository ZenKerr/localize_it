use crate::{
    backends::expression::{arguments::Arguments, static_expression::static_expression},
    utils::NamesProvider,
};
use proc_macro::TokenStream;
use syn::{parse, Result};

pub fn backend(input: TokenStream) -> Result<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let expression = static_expression(arguments, names_provider);

    Ok(expression.into())
}
