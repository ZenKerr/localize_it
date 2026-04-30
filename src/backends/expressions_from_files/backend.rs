use crate::{
    backends::expressions_from_files::{
        arguments::Arguments, static_expressions::static_expressions,
    },
    utils::NamesProvider,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Result};

pub fn backend(input: TokenStream) -> Result<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let expressions = static_expressions(arguments, names_provider)?;

    Ok(quote! {
        #expressions
    }
    .into())
}
