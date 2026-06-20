//! Build-time Rust code generation for typed theme-token contracts.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use spectrum_core::{Color, ShadowLayer};
use spectrum_resolver::{ColorBinding, ResolvedTheme, resolve_theme};
use spectrum_schema::{ThemeMode, ThemeSpec};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};
use syn::{
    Attribute, Ident, LitStr, Result as SynResult, Type, Visibility, braced,
    parse::{Parse, ParseStream},
};

mod error;

pub use error::CodegenError;

/// Build-script configuration for generating a typed theme contract.
#[derive(Debug, Clone)]
pub struct ThemeCodegen {
    source_path: PathBuf,
    struct_name: String,
    visibility: String,
    output_file: String,
    facade_path: String,
    emit_rerun_if_changed: bool,
}

impl ThemeCodegen {
    /// Creates a generator for `source_path` that emits `struct_name`.
    pub fn new(source_path: impl Into<PathBuf>, struct_name: impl Into<String>) -> Self {
        Self {
            source_path: source_path.into(),
            struct_name: struct_name.into(),
            visibility: "pub".to_owned(),
            output_file: "theme_tokens.rs".to_owned(),
            facade_path: "::spectrum_theme".to_owned(),
            emit_rerun_if_changed: true,
        }
    }

    /// Sets the generated struct visibility, for example `pub` or `pub(crate)`.
    #[must_use]
    pub fn visibility(mut self, visibility: impl Into<String>) -> Self {
        self.visibility = visibility.into();
        self
    }

    /// Sets the generated file name used by [`Self::generate`].
    #[must_use]
    pub fn output_file(mut self, output_file: impl Into<String>) -> Self {
        self.output_file = output_file.into();
        self
    }

    /// Sets the facade path referenced by generated code.
    #[must_use]
    pub fn facade_path(mut self, facade_path: impl Into<String>) -> Self {
        self.facade_path = facade_path.into();
        self
    }

    /// Enables or disables `cargo:rerun-if-changed` output.
    #[must_use]
    pub const fn emit_rerun_if_changed(mut self, emit: bool) -> Self {
        self.emit_rerun_if_changed = emit;
        self
    }

    /// Generates the Rust file into `$OUT_DIR`.
    pub fn generate(&self) -> Result<PathBuf, CodegenError> {
        let out_dir = env::var("OUT_DIR").map_err(CodegenError::MissingOutDir)?;
        self.generate_to(Path::new(&out_dir).join(&self.output_file))
    }

    /// Generates the Rust file at an explicit output path.
    pub fn generate_to(&self, output_path: impl AsRef<Path>) -> Result<PathBuf, CodegenError> {
        let output_path = output_path.as_ref();
        let code = self.generate_string()?;
        fs::write(output_path, code).map_err(|source| CodegenError::WriteOutput {
            path: output_path.to_owned(),
            source,
        })?;
        if self.emit_rerun_if_changed {
            println!("cargo:rerun-if-changed={}", self.source_path.display());
        }
        Ok(output_path.to_owned())
    }

    /// Generates Rust source code as a string.
    pub fn generate_string(&self) -> Result<String, CodegenError> {
        let visibility: Visibility =
            syn::parse_str(&self.visibility).map_err(CodegenError::InvalidContract)?;
        let name =
            syn::parse_str::<Ident>(&self.struct_name).map_err(CodegenError::InvalidContract)?;
        let facade = syn::parse_str::<TokenStream2>(&self.facade_path)
            .map_err(CodegenError::InvalidContract)?;
        let resolved = resolve_theme_file(&self.source_path)?;
        let tokens = tokens_from_theme(&resolved, Span::call_site(), &facade)?;
        let schema = ThemeSchema(Vec::new(), visibility, name, tokens);
        Ok(expand_schema(schema, Some(resolved), &facade).to_string())
    }
}

/// Parsed schema for an inline typed token contract.
pub struct ThemeSchema(Vec<Attribute>, Visibility, Ident, Vec<Token>);

enum Token {
    Value(Ident, Box<Type>),
    Group(Ident, Vec<Token>),
}

impl Parse for ThemeSchema {
    fn parse(input: ParseStream<'_>) -> SynResult<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        input.parse::<syn::Token![struct]>()?;
        let name = input.parse()?;
        let content;
        braced!(content in input);
        Ok(Self(attrs, visibility, name, parse_tokens(&content)?))
    }
}

fn parse_tokens(input: ParseStream<'_>) -> SynResult<Vec<Token>> {
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

/// Expands a parsed schema into Rust tokens.
#[must_use]
pub fn expand_schema(
    schema: ThemeSchema,
    embedded: Option<ResolvedTheme>,
    facade: &TokenStream2,
) -> TokenStream2 {
    let ThemeSchema(attrs, visibility, name, tokens) = schema;
    let mut generated = Vec::new();
    let (fields, values, types) = expand_tokens(
        &name,
        &visibility,
        &tokens,
        &mut Vec::new(),
        &mut generated,
        facade,
        &attrs,
    );
    let reload_assignments = expand_reload(&tokens, &mut Vec::new(), facade);
    let loader = embedded.map(|theme| {
        let seed_update = seed_update_expr(&theme, facade);
        let theme = resolved_theme_expr(&theme, facade);
        quote! {
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

                #[allow(missing_docs)]
                #visibility fn try_set_seed(
                    &mut self,
                    seed: #facade::Color,
                ) -> Result<(), #facade::ThemeBuildError> {
                    #seed_update
                }
            }
        }
    });

    quote! {
        #(#generated)*
        #(#attrs)*
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

            #[allow(missing_docs)]
            #visibility fn reload<S: #facade::__private::TokenSource>(
                &mut self,
                source: &S,
            ) -> Result<(), S::Error>
            where
                #(#types: #facade::__private::TokenValue<S>,)*
            {
                #(#reload_assignments)*
                Ok(())
            }
        }
        #loader
    }
}

fn resolve_theme_file(path: &Path) -> Result<ResolvedTheme, CodegenError> {
    let source = fs::read_to_string(path).map_err(|source| CodegenError::ReadSource {
        path: path.to_owned(),
        source,
    })?;
    let spec: ThemeSpec = toml::from_str(&source).map_err(|source| CodegenError::ParseToml {
        path: path.to_owned(),
        source,
    })?;
    resolve_theme(&spec).map_err(|source| CodegenError::Resolve {
        path: path.to_owned(),
        source,
    })
}

fn tokens_from_theme(
    theme: &ResolvedTheme,
    span: Span,
    facade: &TokenStream2,
) -> Result<Vec<Token>, CodegenError> {
    let color: Type = syn::parse2(quote!(#facade::Color)).map_err(CodegenError::InvalidContract)?;
    let length: Type =
        syn::parse2(quote!(#facade::Length)).map_err(CodegenError::InvalidContract)?;
    let radius: Type =
        syn::parse2(quote!(#facade::Radius)).map_err(CodegenError::InvalidContract)?;
    let font_weight: Type =
        syn::parse2(quote!(#facade::FontWeight)).map_err(CodegenError::InvalidContract)?;
    let font_style: Type =
        syn::parse2(quote!(#facade::FontStyle)).map_err(CodegenError::InvalidContract)?;
    let line_height: Type =
        syn::parse2(quote!(#facade::LineHeight)).map_err(CodegenError::InvalidContract)?;
    let shadow: Type =
        syn::parse2(quote!(#facade::ShadowLayer)).map_err(CodegenError::InvalidContract)?;
    let entries = theme
        .colors
        .keys()
        .map(|path| (token_segments(path), color.clone()))
        .chain(
            theme
                .lengths
                .keys()
                .map(|path| (token_segments(path), length.clone())),
        )
        .chain(
            theme
                .radii
                .keys()
                .map(|path| (token_segments(path), radius.clone())),
        )
        .chain(
            theme
                .font_weights
                .keys()
                .map(|path| (token_segments(path), font_weight.clone())),
        )
        .chain(
            theme
                .font_styles
                .keys()
                .map(|path| (token_segments(path), font_style.clone())),
        )
        .chain(
            theme
                .line_heights
                .keys()
                .map(|path| (token_segments(path), line_height.clone())),
        )
        .chain(
            theme
                .shadows
                .iter()
                .map(|(path, _)| (token_segments(path), shadow.clone())),
        )
        .collect::<Vec<_>>();
    file_tokens(&entries, span).map_err(CodegenError::InvalidContract)
}

fn seed_update_expr(theme: &ResolvedTheme, facade: &TokenStream2) -> TokenStream2 {
    let bindings = theme
        .colors
        .iter()
        .filter_map(|(path, binding)| match binding {
            ColorBinding::Material(role) => Some((path, role)),
            ColorBinding::Color(_) => None,
        });
    let mut bindings = bindings.peekable();
    let Some((first_path, _)) = bindings.peek() else {
        return quote!(Ok(()));
    };
    let mode = match theme.meta.mode {
        ThemeMode::Dark => quote!(#facade::__private::ThemeMode::Dark),
        ThemeMode::Light => quote!(#facade::__private::ThemeMode::Light),
    };
    let first_path = LitStr::new(first_path, Span::call_site());
    let updates = bindings.map(|(path, role)| {
        let fields = path
            .split('.')
            .map(|segment| syn::parse_str::<Ident>(segment).expect("validated token path"));
        let role = role.name();
        quote! {
            self.#(#fields).* = material.resolve(
                #facade::__private::MaterialColor::from_name(#role)
                    .expect("embedded Material role was validated")
            );
        }
    });
    quote! {
        let material = #facade::__private::material_colors(seed, #mode, #first_path)?;
        #(#updates)*
        Ok(())
    }
}

fn token_segments(path: &str) -> Vec<String> {
    path.split('.').map(str::to_owned).collect()
}

fn file_tokens(paths: &[(Vec<String>, Type)], span: Span) -> SynResult<Vec<Token>> {
    let mut grouped = BTreeMap::<String, Vec<(Vec<String>, Type)>>::new();
    for (path, ty) in paths {
        let (head, tail) = path.split_first().expect("split path has a segment");
        grouped
            .entry(head.clone())
            .or_default()
            .push((tail.to_vec(), ty.clone()));
    }
    grouped
        .into_iter()
        .map(|(name, children)| {
            let name = syn::parse_str::<Ident>(&name)
                .map_err(|_| syn::Error::new(span, "invalid Rust token path"))?;
            if children.len() == 1 && children[0].0.is_empty() {
                Ok(Token::Value(name, Box::new(children[0].1.clone())))
            } else if children.iter().any(|(path, _)| path.is_empty()) {
                Err(syn::Error::new(span, "token path is both value and group"))
            } else {
                Ok(Token::Group(name, file_tokens(&children, span)?))
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
    let lengths = theme.lengths.iter().map(|(path, length)| {
        let path = LitStr::new(path, Span::call_site());
        let length = length.to_string();
        quote!((
            #path.to_owned(),
            #length.parse::<#facade::Length>()
                .expect("embedded length was validated at compile time")
        ))
    });
    let radii = theme.radii.iter().map(|(path, radius)| {
        let path = LitStr::new(path, Span::call_site());
        let radius = radius.to_string();
        quote!((
            #path.to_owned(),
            #radius.parse::<#facade::Radius>()
                .expect("embedded radius was validated at compile time")
        ))
    });
    let font_weights = theme.font_weights.iter().map(|(path, weight)| {
        let path = LitStr::new(path, Span::call_site());
        let weight = weight.value();
        quote!((
            #path.to_owned(),
            #facade::FontWeight::new(#weight)
                .expect("embedded font weight was validated at compile time")
        ))
    });
    let font_styles = theme.font_styles.iter().map(|(path, style)| {
        let path = LitStr::new(path, Span::call_site());
        let style = style.to_string();
        quote!((
            #path.to_owned(),
            #style.parse::<#facade::FontStyle>()
                .expect("embedded font style was validated at compile time")
        ))
    });
    let line_heights = theme.line_heights.iter().map(|(path, line_height)| {
        let path = LitStr::new(path, Span::call_site());
        let line_height = line_height.to_string();
        quote!((
            #path.to_owned(),
            #line_height.parse::<#facade::LineHeight>()
                .expect("embedded line height was validated at compile time")
        ))
    });
    let shadows = theme.shadows.iter().map(|(path, shadow)| {
        let path = LitStr::new(path, Span::call_site());
        let shadow = shadow_expr(*shadow, facade);
        quote!((#path.to_owned(), #shadow))
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
            lengths: ::std::collections::BTreeMap::from([#(#lengths),*]),
            radii: ::std::collections::BTreeMap::from([#(#radii),*]),
            font_weights: ::std::collections::BTreeMap::from([#(#font_weights),*]),
            font_styles: ::std::collections::BTreeMap::from([#(#font_styles),*]),
            line_heights: ::std::collections::BTreeMap::from([#(#line_heights),*]),
            shadows: ::std::vec![#(#shadows),*],
        }
    }
}

fn shadow_expr(shadow: ShadowLayer, facade: &TokenStream2) -> TokenStream2 {
    let color = color_expr(shadow.color(), facade);
    let lengths = [
        shadow.offset_x(),
        shadow.offset_y(),
        shadow.blur(),
        shadow.spread(),
    ]
    .map(|length| length.to_string());
    let [offset_x, offset_y, blur, spread] = lengths;
    quote!(#facade::ShadowLayer::new(
        #color,
        #offset_x.parse().expect("embedded shadow offset was validated"),
        #offset_y.parse().expect("embedded shadow offset was validated"),
        #blur.parse().expect("embedded shadow blur was validated"),
        #spread.parse().expect("embedded shadow spread was validated"),
    ).expect("embedded shadow was validated"))
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
    struct_attrs: &[Attribute],
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
                let (group_fields, group_values, group_types) = expand_tokens(
                    root,
                    visibility,
                    tokens,
                    path,
                    generated,
                    facade,
                    struct_attrs,
                );
                path.pop();
                generated.push(quote! {
                    #[doc(hidden)]
                    #[allow(missing_docs)]
                    #(#struct_attrs)*
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

fn expand_reload(
    tokens: &[Token],
    path: &mut Vec<Ident>,
    facade: &TokenStream2,
) -> Vec<TokenStream2> {
    let mut assignments = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                let token_path = LitStr::new(&token_path(path, name), Span::call_site());

                let self_path = if path.is_empty() {
                    quote!(self.#name)
                } else {
                    let fields = path.iter();
                    quote!(self.#(#fields).*.#name)
                };

                assignments.push(quote!(
                    #self_path = <#ty as #facade::__private::TokenValue<S>>::read(
                        source,
                        #token_path,
                    )?;
                ));
            }
            Token::Group(name, tokens) => {
                path.push(name.clone());
                assignments.extend(expand_reload(tokens, path, facade));
                path.pop();
            }
        }
    }
    assignments
}

fn token_path(path: &[Ident], name: &Ident) -> String {
    path.iter()
        .chain(core::iter::once(name))
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(".")
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
    format_ident!("{root}{suffix}")
}
