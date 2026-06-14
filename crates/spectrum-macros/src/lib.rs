//! Procedural macros for generating strongly typed theme token structures.

use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Ident, LitStr, Result, Type, Visibility, braced, parse::Parse, parse::ParseStream,
    parse_macro_input,
};

struct ThemeSchema(Visibility, Ident, Vec<Token>);

enum Token {
    Value(Ident, Type),
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
            tokens.push(Token::Value(name, input.parse()?));
        } else {
            let content;
            braced!(content in input);
            tokens.push(Token::Group(name, parse_tokens(&content)?));
        }
        let _ = input.parse::<syn::Token![,]>();
    }
    Ok(tokens)
}

/// Defines a strongly typed theme-token contract from an inline schema.
#[proc_macro]
pub fn define_theme_tokens(input: TokenStream) -> TokenStream {
    let schema = parse_macro_input!(input as ThemeSchema);
    let mut generated = Vec::new();
    let facade = facade_path();
    let (fields, values, types) = expand_tokens(
        &schema.1,
        &schema.0,
        &schema.2,
        &mut Vec::new(),
        &mut generated,
        &facade,
    );
    let visibility = &schema.0;
    let name = &schema.1;

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
    }
    .into()
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
                types.push(ty.clone());
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
