use crate::{
    backends::expressions_from_files::{
        arguments::Arguments, static_expressions::static_expressions,
    },
    utils::{aliases::SynResult, NamesProvider},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse;

pub fn backend(input: TokenStream) -> SynResult<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let expressions = static_expressions(arguments, names_provider)?;

    Ok(quote! {
        #expressions
    }
    .into())
}
