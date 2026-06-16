//! Procedural macros for generating strongly typed theme token structures.

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use spectrum_codegen::{ThemeSchema, expand_schema};
use syn::{Ident, parse_macro_input};

/// Defines a strongly typed theme-token contract from an inline schema.
#[proc_macro]
pub fn define_theme_tokens(input: TokenStream) -> TokenStream {
    let schema = parse_macro_input!(input as ThemeSchema);
    expand_schema(schema, None, &facade_path()).into()
}

fn facade_path() -> TokenStream2 {
    if let Ok(FoundCrate::Name(name)) = crate_name("spectrum-theme") {
        let name = Ident::new(&name.replace('-', "_"), Span::call_site());
        quote!(::#name)
    } else {
        quote!(::spectrum_theme)
    }
}
