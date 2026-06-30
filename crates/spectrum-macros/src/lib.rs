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
/// # Reusable component state sets
///
/// `component` defines one reusable token struct. A contract can instantiate it
/// directly for stateless tokens, or use `states` for named UI states where
/// every state has the same Rust type:
///
/// ```ignore
/// use spectrum_theme::{define_theme_tokens, Color};
///
/// define_theme_tokens! {
///     #[derive(Clone, Debug, PartialEq)]
///     pub struct MyTheme {
///         component ButtonTokens {
///             fg: Color,
///             bg: Color,
///         }
///
///         button: ButtonTokens,
///
///         states nav_button: ButtonTokens {
///             normal,
///             hover extends normal,
///             press_down extends hover,
///             focus extends normal,
///         }
///
///         states secondary_nav_button inherit nav_button,
///     }
/// }
/// ```
///
/// This generates `ButtonTokens`, a `button: ButtonTokens` field,
/// `MyThemeNavButtonStates`, and `MyThemeNavButtonState`. The state container
/// exposes `get(state)`, and the state enum exposes `parent()` for declared
/// `extends` relationships. Parent states must be declared in the same state
/// set; duplicate states and inheritance cycles are rejected while parsing the
/// contract. `states secondary_nav_button inherit nav_button` copies the
/// component type and the complete state list from `nav_button` at contract
/// generation time; token values still use `secondary_nav_button.*` paths.
///
/// # Constructing from a token source
///
/// The generated struct has `try_from_source` for one-shot construction and
/// `reload` for in-place updates from any type that implements
/// `spectrum_theme::source::TokenSource` plus `ThemeValue<Source>` for each
/// token value type used by the contract.
#[proc_macro]
pub fn define_theme_tokens(input: TokenStream) -> TokenStream {
    let schema = parse_macro_input!(input as ThemeSchema);
    expand_schema(schema, &facade_path()).into()
}

fn facade_path() -> TokenStream2 {
    if let Ok(FoundCrate::Name(name)) = crate_name("spectrum-theme") {
        let name = Ident::new(&name.replace('-', "_"), Span::call_site());
        quote!(::#name)
    } else {
        quote!(::spectrum_theme)
    }
}
