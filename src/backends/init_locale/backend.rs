use crate::{
    backends::init_locale::{
        arguments::Arguments,
        parts::{
            enum_locale, macro_expression, macro_expression_part, macro_expression_parts,
            macro_expressions, macro_expressions_from_files, macro_localize, mod_storage,
        },
    },
    utils::{NamesProvider, aliases::LocalizeItResult},
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse;

pub fn backend(input: TokenStream) -> LocalizeItResult<TokenStream> {
    let arguments = &parse::<Arguments>(input)?;
    let names_provider = &NamesProvider::new(arguments.path.clone());

    let locale = enum_locale(arguments)?;
    let storage = mod_storage(arguments);
    let expression = macro_expression(arguments, names_provider)?;
    let expressions = macro_expressions(names_provider);
    let localize = macro_localize(arguments, names_provider);
    let expression_part = macro_expression_part(names_provider);
    let expression_parts = macro_expression_parts(names_provider);
    let expressions_from_files = macro_expressions_from_files(arguments, names_provider)?;

    Ok(quote! {
        #locale
        #storage
        #expression
        #expressions
        #localize
        #expression_part
        #expression_parts
        #expressions_from_files
    }
    .into())
}
