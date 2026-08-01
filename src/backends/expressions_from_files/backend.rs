use crate::{
    backends::expressions_from_files::{
        arguments::Arguments, static_expressions::static_expressions,
    },
    utils::aliases::LocalizeItResult,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse;

pub fn backend(input: TokenStream) -> LocalizeItResult<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;

    let expressions = static_expressions(arguments)?;

    Ok(quote! {
        #expressions
    }
    .into())
}
