use crate::{
    backends::expressions_from_files::{arguments::Arguments, parts::static_expression},
    utils::NamesProvider,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Error};

pub fn backend(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse(input)?;
    let arguments = &Arguments::new(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let expressions = arguments
        .expressions
        .iter()
        .map(|expression| static_expression(expression, arguments, names_provider));

    Ok(quote! {
        #(#expressions)*
    }
    .into())
}
