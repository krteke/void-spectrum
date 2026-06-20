//! Procedural macros for generating strongly typed theme token structures.

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use spectrum_codegen::{ThemeSchema, expand_schema};
use syn::{Ident, parse_macro_input};

/// Defines a strongly typed theme-token contract from an inline schema.
///
/// # Generated struct
///
/// Each group becomes a nested struct, each leaf becomes a `pub` field:
///
/// ```ignore
/// use spectrum_theme::{define_theme_tokens, Color, Length};
///
/// define_theme_tokens! {
///     pub struct MyTheme {
///         surface {
///             background: Color,
///         }
///         spacing {
///             gap: Length,
///         }
///     }
/// }
/// ```
///
/// Generates `MyTheme` with `pub surface: MyThemeSurface` and
/// `pub spacing: MyThemeSpacing`.
///
/// # User attributes
///
/// Outer attributes placed before `struct` are applied to **every** generated
/// struct (the root struct and all nested sub-structs):
///
/// ```ignore
/// use spectrum_theme::{define_theme_tokens, Color};
///
/// define_theme_tokens! {
///     #[derive(Clone, Debug)]
///     pub struct MyTheme {
///         surface {
///             background: Color,
///         }
///     }
/// }
/// ```
///
/// `MyTheme` and `MyThemeSurface` both receive `#[derive(Clone, Debug)]`.
/// Multiple `#[derive]` attributes and split attributes (`#[derive(Clone)] #[derive(Debug)]`)
/// are supported.
///
/// # Constructing from a token source
///
/// The generated struct has `try_from_source` for one-shot construction and
/// `reload` for in-place updates from any type that implements
/// `spectrum_theme::__private::TokenSource` and the per-type `*Source` traits.
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
