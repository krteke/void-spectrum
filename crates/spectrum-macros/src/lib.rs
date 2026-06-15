//! Procedural macros for generating strongly typed theme token structures.

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use spectrum_core::Color;
use spectrum_resolver::{ColorBinding, ResolvedTheme, resolve_theme};
use spectrum_schema::{ThemeMode, ThemeSpec};
use std::{collections::BTreeMap, fs};
use syn::{
    Ident, LitStr, Result, Token as SynToken, Type, Visibility, braced, parse::Parse,
    parse::ParseStream, parse_macro_input, spanned::Spanned,
};

struct ThemeSchema(Visibility, Ident, Vec<Token>);
struct IncludeSchema(ThemeSchema, LitStr);

enum Token {
    Value(Ident, Box<Type>),
    Group(Ident, Vec<Token>),
}

impl Parse for ThemeSchema {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let visibility = input.parse()?;
        input.parse::<syn::Token![struct]>()?;
        let name = input.parse()?;
        let content;
        braced!(content in input);
        Ok(Self(visibility, name, parse_tokens(&content)?))
    }
}

fn parse_tokens(input: ParseStream<'_>) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    while !input.is_empty() {
        let name = input.parse()?;
        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            tokens.push(Token::Value(name, Box::new(input.parse()?)));
        } else {
            let content;
            braced!(content in input);
            tokens.push(Token::Group(name, parse_tokens(&content)?));
        }
        let _ = input.parse::<syn::Token![,]>();
    }
    Ok(tokens)
}

impl Parse for IncludeSchema {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let visibility = input.parse()?;
        input.parse::<syn::Token![struct]>()?;
        let name = input.parse()?;
        input.parse::<SynToken![;]>()?;
        parse_word(input, "source")?;
        input.parse::<SynToken![=]>()?;
        let include: syn::Macro = input.parse()?;
        if !include.path.is_ident("include_str") {
            return Err(syn::Error::new(
                include.path.span(),
                "expected `include_str!`",
            ));
        }
        let path = include.parse_body()?;
        input.parse::<SynToken![;]>()?;
        parse_word(input, "format")?;
        input.parse::<SynToken![=]>()?;
        parse_word(input, "toml")?;
        let _ = input.parse::<SynToken![;]>();
        Ok(Self(ThemeSchema(visibility, name, Vec::new()), path))
    }
}

fn parse_word(input: ParseStream<'_>, expected: &str) -> Result<()> {
    let word: Ident = input.parse()?;
    (word == expected)
        .then_some(())
        .ok_or_else(|| syn::Error::new(word.span(), format!("expected `{expected}`")))
}

/// Defines a strongly typed theme-token contract from an inline schema.
#[proc_macro]
pub fn define_theme_tokens(input: TokenStream) -> TokenStream {
    expand_schema(parse_macro_input!(input as ThemeSchema), None).into()
}

/// Defines a strongly typed color-token contract from a TOML theme file.
#[proc_macro]
pub fn include_theme_tokens(input: TokenStream) -> TokenStream {
    let IncludeSchema(schema, path) = parse_macro_input!(input as IncludeSchema);
    match include_schema(schema, path) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn expand_schema(schema: ThemeSchema, embedded: Option<(LitStr, ResolvedTheme)>) -> TokenStream2 {
    let ThemeSchema(visibility, name, tokens) = schema;
    let mut generated = Vec::new();
    let facade = facade_path();
    let (fields, values, types) = expand_tokens(
        &name,
        &visibility,
        &tokens,
        &mut Vec::new(),
        &mut generated,
        &facade,
    );
    let loader = embedded.map(|(path, theme)| {
        let theme = resolved_theme_expr(&theme, &facade);
        quote! {
            const _: &str = include_str!(#path);
            impl #name {
                fn __embedded_theme() -> &'static #facade::__private::ResolvedTheme {
                    static THEME: ::std::sync::OnceLock<#facade::__private::ResolvedTheme> =
                        ::std::sync::OnceLock::new();
                    THEME.get_or_init(|| #theme)
                }

                #[allow(missing_docs)]
                #visibility fn try_load() -> Result<Self, #facade::ThemeBuildError> {
                    Self::try_from_source(Self::__embedded_theme())
                }

                #[allow(missing_docs)]
                #visibility fn try_load_with_seed(
                    seed: #facade::Color,
                ) -> Result<Self, #facade::ThemeBuildError> {
                    let source =
                        #facade::__private::SeededTheme::new(Self::__embedded_theme(), seed);
                    Self::try_from_source(&source)
                }
            }
        }
    });

    quote! {
        #(#generated)*
        #[allow(missing_docs)]
        #visibility struct #name {
            #(#fields)*
        }
        impl #name {
            #[allow(missing_docs)]
            #visibility fn try_from_source<S: #facade::__private::TokenSource>(
                source: &S,
            ) -> Result<Self, S::Error>
            where
                #(#types: #facade::__private::TokenValue<S>,)*
            {
                Ok(Self { #(#values)* })
            }
        }
        #loader
    }
}

fn include_schema(mut schema: ThemeSchema, path: LitStr) -> Result<TokenStream2> {
    let file = path
        .span()
        .local_file()
        .and_then(|source| source.parent().map(|parent| parent.join(path.value())))
        .ok_or_else(|| syn::Error::new(path.span(), "cannot locate macro invocation file"))?;
    let source = fs::read_to_string(file)
        .map_err(|error| syn::Error::new(path.span(), error.to_string()))?;
    let spec: ThemeSpec =
        toml::from_str(&source).map_err(|error| syn::Error::new(path.span(), error.to_string()))?;
    let resolved =
        resolve_theme(&spec).map_err(|error| syn::Error::new(path.span(), error.to_string()))?;
    let paths = resolved
        .colors
        .keys()
        .map(|path| path.split('.').map(str::to_owned).collect())
        .collect::<Vec<Vec<String>>>();
    let facade = facade_path();
    let ty = syn::parse2(quote!(#facade::Color))?;
    schema.2 = file_tokens(&paths, &ty, path.span())?;
    Ok(expand_schema(schema, Some((path, resolved))))
}

fn file_tokens(paths: &[Vec<String>], ty: &Type, span: Span) -> Result<Vec<Token>> {
    let mut grouped = BTreeMap::<String, Vec<Vec<String>>>::new();
    for path in paths {
        let (head, tail) = path.split_first().expect("split path has a segment");
        grouped.entry(head.clone()).or_default().push(tail.to_vec());
    }
    grouped
        .into_iter()
        .map(|(name, children)| {
            let name = syn::parse_str::<Ident>(&name)
                .map_err(|_| syn::Error::new(span, "invalid Rust token path"))?;
            if children.len() == 1 && children[0].is_empty() {
                Ok(Token::Value(name, Box::new(ty.clone())))
            } else if children.iter().any(Vec::is_empty) {
                Err(syn::Error::new(span, "token path is both value and group"))
            } else {
                Ok(Token::Group(name, file_tokens(&children, ty, span)?))
            }
        })
        .collect()
}

fn resolved_theme_expr(theme: &ResolvedTheme, facade: &TokenStream2) -> TokenStream2 {
    let meta = &theme.meta;
    let name = &meta.name;
    let author = option_string(meta.author.as_ref());
    let version = option_string(meta.version.as_ref());
    let description = option_string(meta.description.as_ref());
    let mode = match meta.mode {
        ThemeMode::Dark => quote!(#facade::__private::ThemeMode::Dark),
        ThemeMode::Light => quote!(#facade::__private::ThemeMode::Light),
    };
    let seed = option_color(theme.seed, facade);
    let colors = theme.colors.iter().map(|(path, binding)| {
        let path = LitStr::new(path, Span::call_site());
        let binding = binding_expr(*binding, facade);
        quote!((#path.to_owned(), #binding))
    });

    quote! {
        #facade::__private::ResolvedTheme {
            meta: #facade::__private::ThemeMeta {
                name: #name.to_owned(),
                author: #author,
                mode: #mode,
                version: #version,
                description: #description,
            },
            seed: #seed,
            colors: ::std::collections::BTreeMap::from([#(#colors),*]),
        }
    }
}

fn option_string(value: Option<&String>) -> TokenStream2 {
    value.map_or_else(|| quote!(None), |value| quote!(Some(#value.to_owned())))
}

fn option_color(value: Option<Color>, facade: &TokenStream2) -> TokenStream2 {
    value.map_or_else(
        || quote!(None),
        |color| {
            let color = color_expr(color, facade);
            quote!(Some(#color))
        },
    )
}

fn binding_expr(binding: ColorBinding, facade: &TokenStream2) -> TokenStream2 {
    match binding {
        ColorBinding::Color(color) => {
            let color = color_expr(color, facade);
            quote!(#facade::__private::ColorBinding::Color(#color))
        }
        ColorBinding::Material(role) => {
            let role = role.name();
            quote!(
                #facade::__private::ColorBinding::Material(
                    #facade::__private::MaterialColor::from_name(#role)
                        .expect("embedded Material role was validated at compile time")
                )
            )
        }
    }
}

fn color_expr(color: Color, facade: &TokenStream2) -> TokenStream2 {
    let (red, green, blue, alpha) = (color.red(), color.green(), color.blue(), color.alpha());
    match color {
        Color::Rgb(_) => quote!(#facade::Color::new(#red, #green, #blue)),
        Color::Rgba(_) => quote!(#facade::Color::new_rgba(#red, #green, #blue, #alpha)),
    }
}

fn expand_tokens(
    root: &Ident,
    visibility: &Visibility,
    tokens: &[Token],
    path: &mut Vec<Ident>,
    generated: &mut Vec<TokenStream2>,
    facade: &TokenStream2,
) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<Type>) {
    let mut fields = Vec::new();
    let mut values = Vec::new();
    let mut types = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                let token_path = LitStr::new(&token_path(path, name), Span::call_site());
                fields.push(quote!(pub #name: #ty,));
                types.push(ty.as_ref().clone());
                values.push(quote!(
                    #name: <#ty as #facade::__private::TokenValue<S>>::read(source, #token_path)?,
                ));
            }
            Token::Group(name, tokens) => {
                path.push(name.clone());
                let group_name = group_name(root, path);
                let (group_fields, group_values, group_types) =
                    expand_tokens(root, visibility, tokens, path, generated, facade);
                path.pop();
                generated.push(quote! {
                    #[doc(hidden)]
                    #[allow(missing_docs)]
                    #visibility struct #group_name {
                        #(#group_fields)*
                    }
                });
                fields.push(quote!(pub #name: #group_name,));
                values.push(quote!(#name: #group_name { #(#group_values)* },));
                types.extend(group_types);
            }
        }
    }
    (fields, values, types)
}

fn token_path(path: &[Ident], name: &Ident) -> String {
    path.iter()
        .chain(core::iter::once(name))
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

fn facade_path() -> TokenStream2 {
    if let Ok(FoundCrate::Name(name)) = crate_name("spectrum-theme") {
        let name = Ident::new(&name.replace('-', "_"), Span::call_site());
        quote!(::#name)
    } else {
        quote!(::spectrum_theme)
    }
}

fn group_name(root: &Ident, path: &[Ident]) -> Ident {
    let suffix: String = path
        .iter()
        .map(|part| {
            let mut name = part.to_string();
            let _ = name.get_mut(0..1).map(str::make_ascii_uppercase);
            name.replace('_', "")
        })
        .collect();
    format_ident!("{root}{suffix}Tokens")
}
