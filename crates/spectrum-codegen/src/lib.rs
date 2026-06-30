//! Build-time Rust code generation for typed theme-token contracts.

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use std::{
    collections::{BTreeMap, BTreeSet},
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
    contract_path: PathBuf,
    visibility: String,
    output_file: String,
    facade_path: String,
    emit_rerun_if_changed: bool,
}

impl ThemeCodegen {
    /// Creates a generator from an external contract and a contract-aware TOML values file.
    pub fn from_contract(
        contract_path: impl Into<PathBuf>,
        values_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            source_path: values_path.into(),
            contract_path: contract_path.into(),
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
            println!("cargo:rerun-if-changed={}", self.contract_path.display());
        }
        Ok(output_path.to_owned())
    }

    /// Generates Rust source code as a string.
    pub fn generate_string(&self) -> Result<String, CodegenError> {
        let facade = syn::parse_str::<TokenStream2>(&self.facade_path)
            .map_err(CodegenError::InvalidContract)?;
        self.generate_contract_string(&facade)
    }

    fn generate_contract_string(&self, facade: &TokenStream2) -> Result<String, CodegenError> {
        let contract =
            fs::read_to_string(&self.contract_path).map_err(|source| CodegenError::ReadSource {
                path: self.contract_path.clone(),
                source,
            })?;
        let schema: ThemeSchema =
            syn::parse_str(&contract).map_err(CodegenError::InvalidContract)?;
        let values =
            fs::read_to_string(&self.source_path).map_err(|source| CodegenError::ReadSource {
                path: self.source_path.clone(),
                source,
            })?;
        values
            .parse::<toml::Table>()
            .map_err(|source| CodegenError::ParseToml {
                path: self.source_path.clone(),
                source,
            })?;
        Ok(expand_schema_inner(schema, Some(&values), facade).to_string())
    }
}

/// Parsed schema for an inline typed token contract.
pub struct ThemeSchema(Vec<Attribute>, Visibility, Ident, Vec<Token>);

#[derive(Clone)]
enum Token {
    Value(Ident, Box<Type>),
    Group(Ident, Vec<Token>),
    Component(Ident, Vec<Token>),
    States(StateSet),
}

#[derive(Clone)]
struct StateSet {
    name: Ident,
    component: Ident,
    states: Vec<StateVariant>,
}

#[derive(Clone)]
struct StateVariant {
    name: Ident,
    extends: Option<Ident>,
}

mod keyword {
    syn::custom_keyword!(component);
    syn::custom_keyword!(states);
    syn::custom_keyword!(extends);
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
        if input.peek(keyword::component) {
            input.parse::<keyword::component>()?;
            let name = input.parse()?;
            let content;
            braced!(content in input);
            tokens.push(Token::Component(name, parse_tokens(&content)?));
            let _ = input.parse::<syn::Token![,]>();
            continue;
        }

        if input.peek(keyword::states) {
            input.parse::<keyword::states>()?;
            let name = input.parse()?;
            input.parse::<syn::Token![:]>()?;
            let component = input.parse()?;
            let content;
            braced!(content in input);
            tokens.push(Token::States(StateSet {
                name,
                component,
                states: parse_states(&content)?,
            }));
            let _ = input.parse::<syn::Token![,]>();
            continue;
        }

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

fn parse_states(input: ParseStream<'_>) -> SynResult<Vec<StateVariant>> {
    let mut states = Vec::new();
    while !input.is_empty() {
        let name = input.parse()?;
        let extends = if input.peek(keyword::extends) {
            input.parse::<keyword::extends>()?;
            Some(input.parse()?)
        } else {
            None
        };
        states.push(StateVariant { name, extends });
        let _ = input.parse::<syn::Token![,]>();
    }
    validate_states(&states)?;
    Ok(states)
}

fn validate_states(states: &[StateVariant]) -> SynResult<()> {
    let mut declared = BTreeSet::new();
    for state in states {
        if !declared.insert(state.name.to_string()) {
            return Err(syn::Error::new(
                state.name.span(),
                format!("duplicate state `{}` in state set", state.name),
            ));
        }
    }
    for state in states {
        if let Some(parent) = &state.extends
            && !declared.contains(&parent.to_string())
        {
            return Err(syn::Error::new(
                parent.span(),
                format!("unknown parent state `{parent}`"),
            ));
        }
    }
    for state in states {
        validate_state_chain(states, state)?;
    }
    Ok(())
}

fn validate_state_chain(states: &[StateVariant], state: &StateVariant) -> SynResult<()> {
    let mut seen = BTreeSet::new();
    let mut current = Some(state.name.clone());
    while let Some(name) = current {
        if !seen.insert(name.to_string()) {
            return Err(syn::Error::new(
                name.span(),
                format!("state inheritance cycle involving `{name}`"),
            ));
        }
        current = states
            .iter()
            .find(|candidate| candidate.name == name)
            .and_then(|candidate| candidate.extends.clone());
    }
    Ok(())
}

/// Expands a parsed schema into Rust tokens.
#[must_use]
pub fn expand_schema(schema: ThemeSchema, facade: &TokenStream2) -> TokenStream2 {
    expand_schema_inner(schema, None, facade)
}

fn expand_schema_inner(
    schema: ThemeSchema,
    embedded_toml: Option<&str>,
    facade: &TokenStream2,
) -> TokenStream2 {
    let ThemeSchema(attrs, visibility, name, tokens) = schema;
    if let Err(error) = validate_schema(&tokens) {
        return error.to_compile_error();
    }
    let components = collect_components(&tokens);
    let mut generated = Vec::new();
    let env = ExpandEnv {
        components: &components,
        facade,
        struct_attrs: &attrs,
    };
    let (fields, values, types) = expand_tokens(
        &name,
        &visibility,
        &tokens,
        &env,
        &mut Vec::new(),
        &mut generated,
    );
    let reload_assignments = expand_reload(&tokens, &components, &mut Vec::new(), facade);
    let loader = embedded_toml.map(|source| embedded_loader(&name, &visibility, source, facade));

    quote! {
        #(#generated)*
        #(#attrs)*
        #[allow(missing_docs)]
        #visibility struct #name {
            #(#fields)*
        }
        impl #name {
            #[allow(missing_docs)]
            #visibility fn try_from_source<S: #facade::source::TokenSource>(
                source: &S,
            ) -> Result<Self, S::Error>
            where
                #(#types: #facade::source::ThemeValue<S>,)*
            {
                Ok(Self { #(#values)* })
            }

            #[allow(missing_docs)]
            #visibility fn reload<S: #facade::source::TokenSource>(
                &mut self,
                source: &S,
            ) -> Result<(), S::Error>
            where
                #(#types: #facade::source::ThemeValue<S>,)*
            {
                #(#reload_assignments)*
                Ok(())
            }
        }
        #loader
    }
}

fn embedded_loader(
    name: &Ident,
    visibility: &Visibility,
    source: &str,
    facade: &TokenStream2,
) -> TokenStream2 {
    let source = LitStr::new(source, Span::call_site());
    quote! {
        impl #name {
            fn __embedded_source() -> &'static #facade::config::TomlThemeSource {
                static SOURCE: ::std::sync::OnceLock<#facade::config::TomlThemeSource> =
                    ::std::sync::OnceLock::new();
                SOURCE.get_or_init(|| {
                    #facade::config::TomlThemeSource::parse(#source)
                        .expect("embedded TOML theme source was validated at compile time")
                })
            }

            #[allow(missing_docs)]
            #visibility fn try_load() -> Result<Self, #facade::ThemeBuildError> {
                Self::try_from_source(Self::__embedded_source())
            }
        }
    }
}

fn validate_schema(tokens: &[Token]) -> SynResult<()> {
    let mut components = BTreeSet::new();
    collect_component_names(tokens, &mut components)?;
    validate_state_components(tokens, &components)
}

fn collect_component_names(tokens: &[Token], components: &mut BTreeSet<String>) -> SynResult<()> {
    for token in tokens {
        match token {
            Token::Component(name, children) => {
                if !components.insert(name.to_string()) {
                    return Err(syn::Error::new(
                        name.span(),
                        format!("duplicate component `{name}`"),
                    ));
                }
                collect_component_names(children, components)?;
            }
            Token::Group(_, children) => collect_component_names(children, components)?,
            Token::Value(_, _) | Token::States(_) => {}
        }
    }
    Ok(())
}

fn validate_state_components(tokens: &[Token], components: &BTreeSet<String>) -> SynResult<()> {
    for token in tokens {
        match token {
            Token::States(states) if !components.contains(&states.component.to_string()) => {
                return Err(syn::Error::new(
                    states.component.span(),
                    format!("unknown state component `{}`", states.component),
                ));
            }
            Token::Group(_, children) | Token::Component(_, children) => {
                validate_state_components(children, components)?;
            }
            Token::Value(_, _) | Token::States(_) => {}
        }
    }
    Ok(())
}

fn collect_components(tokens: &[Token]) -> BTreeMap<String, Vec<Token>> {
    let mut components = BTreeMap::new();
    collect_components_into(tokens, &mut components);
    components
}

fn collect_components_into(tokens: &[Token], components: &mut BTreeMap<String, Vec<Token>>) {
    for token in tokens {
        match token {
            Token::Component(name, children) => {
                components.insert(name.to_string(), children.clone());
                collect_components_into(children, components);
            }
            Token::Group(_, children) => collect_components_into(children, components),
            Token::Value(_, _) | Token::States(_) => {}
        }
    }
}

struct ExpandEnv<'a> {
    components: &'a BTreeMap<String, Vec<Token>>,
    facade: &'a TokenStream2,
    struct_attrs: &'a [Attribute],
}

fn expand_tokens(
    root: &Ident,
    visibility: &Visibility,
    tokens: &[Token],
    env: &ExpandEnv<'_>,
    path: &mut Vec<Ident>,
    generated: &mut Vec<TokenStream2>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>, Vec<Type>) {
    let struct_attrs = env.struct_attrs;
    let mut fields = Vec::new();
    let mut values = Vec::new();
    let mut types = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                fields.push(quote!(pub #name: #ty,));
                if let Some((component, component_tokens)) =
                    component_type(ty.as_ref(), env.components)
                {
                    let mut source_path = path.clone();
                    source_path.push(name.clone());
                    let (component_values, component_types) = expand_token_values(
                        component,
                        component_tokens,
                        &mut source_path,
                        &mut Vec::new(),
                        env.components,
                        env.facade,
                        None,
                    );
                    values.push(quote!(#name: #component { #(#component_values)* },));
                    types.extend(component_types);
                } else {
                    let token_path = LitStr::new(&token_path(path, name), Span::call_site());
                    types.push(ty.as_ref().clone());
                    values.push(quote!(
                        #name: source.token::<#ty>(#token_path)?,
                    ));
                }
            }
            Token::Group(name, tokens) => {
                path.push(name.clone());
                let group_name = group_name(root, path);
                let (group_fields, group_values, group_types) =
                    expand_tokens(root, visibility, tokens, env, path, generated);
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
            Token::Component(name, tokens) => {
                let (component_fields, _, component_types) =
                    expand_tokens(name, visibility, tokens, env, &mut Vec::new(), generated);
                generated.push(quote! {
                    #[allow(missing_docs)]
                    #(#struct_attrs)*
                    #visibility struct #name {
                        #(#component_fields)*
                    }
                });
                types.extend(component_types);
            }
            Token::States(states) => {
                let component_tokens = env
                    .components
                    .get(&states.component.to_string())
                    .expect("states component was parsed before expansion");
                let expanded = expand_states(root, visibility, states, component_tokens, path, env);
                generated.push(expanded.generated);
                fields.push(expanded.field);
                values.push(expanded.value);
                types.extend(expanded.types);
            }
        }
    }
    (fields, values, types)
}

struct StateExpansion {
    generated: TokenStream2,
    field: TokenStream2,
    value: TokenStream2,
    types: Vec<Type>,
}

fn expand_states(
    root: &Ident,
    visibility: &Visibility,
    states: &StateSet,
    component_tokens: &[Token],
    path: &[Ident],
    env: &ExpandEnv<'_>,
) -> StateExpansion {
    let struct_attrs = env.struct_attrs;
    let state_set_name = format_ident!("{}{}States", root, pascal_case(&states.name));
    let state_enum_name = format_ident!("{}{}State", root, pascal_case(&states.name));
    let component = &states.component;
    let state_fields = states.states.iter().map(|state| {
        let name = &state.name;
        quote!(pub #name: #component,)
    });
    let state_values = states.states.iter().map(|state| {
        let name = &state.name;
        let mut state_path = path.to_vec();
        state_path.extend([states.name.clone(), state.name.clone()]);
        let inherited_bases = state_source_bases(states, state, path);
        let (values, _) = expand_token_values(
            component,
            component_tokens,
            &mut state_path,
            &mut Vec::new(),
            env.components,
            env.facade,
            Some(&inherited_bases),
        );
        quote!(#name: #component { #(#values)* },)
    });
    let state_variants = states
        .states
        .iter()
        .map(|state| format_ident!("{}", pascal_case(&state.name)));
    let state_get = states.states.iter().map(|state| {
        let field = &state.name;
        let variant = format_ident!("{}", pascal_case(&state.name));
        quote!(#state_enum_name::#variant => &self.#field,)
    });
    let state_parents = states.states.iter().map(|state| {
        let variant = format_ident!("{}", pascal_case(&state.name));
        if let Some(parent) = &state.extends {
            let parent = format_ident!("{}", pascal_case(parent));
            quote!(#state_enum_name::#variant => Some(#state_enum_name::#parent),)
        } else {
            quote!(#state_enum_name::#variant => None,)
        }
    });
    let mut types = Vec::new();
    for state in &states.states {
        let mut state_path = path.to_vec();
        state_path.extend([states.name.clone(), state.name.clone()]);
        let (_, next_types) = expand_token_values(
            component,
            component_tokens,
            &mut state_path,
            &mut Vec::new(),
            env.components,
            env.facade,
            None,
        );
        types.extend(next_types);
    }

    let name = &states.name;
    StateExpansion {
        generated: quote! {
            #[allow(missing_docs)]
            #(#struct_attrs)*
            #visibility struct #state_set_name {
                #(#state_fields)*
            }

            #[allow(missing_docs)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #visibility enum #state_enum_name {
                #(#state_variants,)*
            }

            impl #state_enum_name {
                #[allow(missing_docs)]
                #[must_use]
                #visibility const fn parent(self) -> Option<Self> {
                    match self {
                        #(#state_parents)*
                    }
                }
            }

            impl #state_set_name {
                #[allow(missing_docs)]
                #[must_use]
                #visibility const fn get(&self, state: #state_enum_name) -> &#component {
                    match state {
                        #(#state_get)*
                    }
                }
            }
        },
        field: quote!(pub #name: #state_set_name,),
        value: quote!(#name: #state_set_name { #(#state_values)* },),
        types,
    }
}

fn expand_token_values(
    root: &Ident,
    tokens: &[Token],
    source_path: &mut Vec<Ident>,
    struct_path: &mut Vec<Ident>,
    components: &BTreeMap<String, Vec<Token>>,
    facade: &TokenStream2,
    inherited_bases: Option<&[Vec<Ident>]>,
) -> (Vec<TokenStream2>, Vec<Type>) {
    let mut values = Vec::new();
    let mut types = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                if let Some((component, component_tokens)) = component_type(ty.as_ref(), components)
                {
                    source_path.push(name.clone());
                    struct_path.push(name.clone());
                    let (component_values, component_types) = expand_token_values(
                        component,
                        component_tokens,
                        source_path,
                        struct_path,
                        components,
                        facade,
                        inherited_bases,
                    );
                    struct_path.pop();
                    source_path.pop();
                    values.push(quote!(#name: #component { #(#component_values)* },));
                    types.extend(component_types);
                } else {
                    let path = LitStr::new(&token_path(source_path, name), Span::call_site());
                    let value = if let Some(bases) = inherited_bases.filter(|bases| bases.len() > 1)
                    {
                        let paths = inherited_path_literals(bases, struct_path, name);
                        quote!(#facade::source::read_inherited(source, [#(#paths),*])?)
                    } else {
                        quote!(source.token::<#ty>(#path)?)
                    };
                    values.push(quote!(#name: #value,));
                    types.push(ty.as_ref().clone());
                }
            }
            Token::Group(name, children) => {
                source_path.push(name.clone());
                struct_path.push(name.clone());
                let group = group_name(root, struct_path);
                let (group_values, group_types) = expand_token_values(
                    root,
                    children,
                    source_path,
                    struct_path,
                    components,
                    facade,
                    inherited_bases,
                );
                struct_path.pop();
                source_path.pop();
                values.push(quote!(#name: #group { #(#group_values)* },));
                types.extend(group_types);
            }
            Token::Component(_, _) | Token::States(_) => {}
        }
    }
    (values, types)
}

fn expand_reload(
    tokens: &[Token],
    components: &BTreeMap<String, Vec<Token>>,
    path: &mut Vec<Ident>,
    facade: &TokenStream2,
) -> Vec<TokenStream2> {
    let mut assignments = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                let self_path = if path.is_empty() {
                    quote!(self.#name)
                } else {
                    let fields = path.iter();
                    quote!(self.#(#fields).*.#name)
                };

                if let Some((_, component_tokens)) = component_type(ty.as_ref(), components) {
                    let mut source_path = path.clone();
                    source_path.push(name.clone());
                    assignments.extend(expand_component_reload(
                        component_tokens,
                        &mut source_path,
                        &mut Vec::new(),
                        &self_path,
                        components,
                        facade,
                        None,
                    ));
                } else {
                    let token_path = LitStr::new(&token_path(path, name), Span::call_site());
                    assignments.push(quote!(
                        #self_path = source.token::<#ty>(#token_path)?;
                    ));
                }
            }
            Token::Group(name, tokens) => {
                path.push(name.clone());
                assignments.extend(expand_reload(tokens, components, path, facade));
                path.pop();
            }
            Token::Component(_, _) => {}
            Token::States(states) => {
                let component_tokens = components
                    .get(&states.component.to_string())
                    .expect("states component was parsed before expansion");
                for state in &states.states {
                    let mut source_path = path.clone();
                    source_path.extend([states.name.clone(), state.name.clone()]);
                    let inherited_bases = state_source_bases(states, state, path);
                    let self_path = if path.is_empty() {
                        let name = &states.name;
                        let state = &state.name;
                        quote!(self.#name.#state)
                    } else {
                        let fields = path.iter();
                        let name = &states.name;
                        let state = &state.name;
                        quote!(self.#(#fields).*.#name.#state)
                    };
                    assignments.extend(expand_component_reload(
                        component_tokens,
                        &mut source_path,
                        &mut Vec::new(),
                        &self_path,
                        components,
                        facade,
                        Some(&inherited_bases),
                    ));
                }
            }
        }
    }
    assignments
}

fn expand_component_reload(
    tokens: &[Token],
    source_path: &mut Vec<Ident>,
    struct_path: &mut Vec<Ident>,
    self_path: &TokenStream2,
    components: &BTreeMap<String, Vec<Token>>,
    facade: &TokenStream2,
    inherited_bases: Option<&[Vec<Ident>]>,
) -> Vec<TokenStream2> {
    let mut assignments = Vec::new();
    for token in tokens {
        match token {
            Token::Value(name, ty) => {
                if let Some((_, component_tokens)) = component_type(ty.as_ref(), components) {
                    let nested_self = quote!(#self_path.#name);
                    source_path.push(name.clone());
                    struct_path.push(name.clone());
                    assignments.extend(expand_component_reload(
                        component_tokens,
                        source_path,
                        struct_path,
                        &nested_self,
                        components,
                        facade,
                        inherited_bases,
                    ));
                    struct_path.pop();
                    source_path.pop();
                } else {
                    let path = LitStr::new(&token_path(source_path, name), Span::call_site());
                    let value = if let Some(bases) = inherited_bases.filter(|bases| bases.len() > 1)
                    {
                        let paths = inherited_path_literals(bases, struct_path, name);
                        quote!(#facade::source::read_inherited(source, [#(#paths),*])?)
                    } else {
                        quote!(source.token::<#ty>(#path)?)
                    };
                    assignments.push(quote!(#self_path.#name = #value;));
                }
            }
            Token::Group(name, children) => {
                let nested_self = quote!(#self_path.#name);
                source_path.push(name.clone());
                struct_path.push(name.clone());
                assignments.extend(expand_component_reload(
                    children,
                    source_path,
                    struct_path,
                    &nested_self,
                    components,
                    facade,
                    inherited_bases,
                ));
                struct_path.pop();
                source_path.pop();
            }
            Token::Component(_, _) | Token::States(_) => {}
        }
    }
    assignments
}

fn component_type<'a>(
    ty: &'a Type,
    components: &'a BTreeMap<String, Vec<Token>>,
) -> Option<(&'a Ident, &'a [Token])> {
    let Type::Path(path) = ty else {
        return None;
    };
    if path.qself.is_some() || path.path.segments.len() != 1 {
        return None;
    }
    let segment = path.path.segments.first()?;
    if !matches!(segment.arguments, syn::PathArguments::None) {
        return None;
    }
    let ident = &segment.ident;
    components
        .get(&ident.to_string())
        .map(|tokens| (ident, tokens.as_slice()))
}

fn state_source_bases(states: &StateSet, state: &StateVariant, path: &[Ident]) -> Vec<Vec<Ident>> {
    state_chain(states, state)
        .into_iter()
        .map(|state_name| {
            let mut base = path.to_vec();
            base.extend([states.name.clone(), state_name]);
            base
        })
        .collect()
}

fn state_chain(states: &StateSet, state: &StateVariant) -> Vec<Ident> {
    let mut names = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current = Some(state.name.clone());
    while let Some(name) = current {
        if !seen.insert(name.to_string()) {
            break;
        }
        current = states
            .states
            .iter()
            .find(|candidate| candidate.name == name)
            .and_then(|candidate| candidate.extends.clone());
        names.push(name);
    }
    names
}

fn inherited_path_literals(
    bases: &[Vec<Ident>],
    struct_path: &[Ident],
    name: &Ident,
) -> Vec<LitStr> {
    bases
        .iter()
        .map(|base| {
            let mut path = base.clone();
            path.extend(struct_path.iter().cloned());
            LitStr::new(&token_path(&path, name), Span::call_site())
        })
        .collect()
}

fn token_path(path: &[Ident], name: &Ident) -> String {
    path.iter()
        .chain(core::iter::once(name))
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

fn pascal_case(name: &Ident) -> String {
    name.to_string()
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            first.to_ascii_uppercase().to_string() + chars.as_str()
        })
        .collect()
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
