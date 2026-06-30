//! Behavior tests for build-time typed theme generation.

use spectrum_codegen::{CodegenError, ThemeCodegen, ThemeSchema, expand_schema};
use std::{
    fs,
    path::{Path, PathBuf},
};

use proc_macro2::TokenStream;

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../spectrum-theme/tests")
        .join(path)
}

fn facade() -> TokenStream {
    syn::parse_str("::spectrum_theme").expect("valid facade path")
}

fn parse_schema_error(input: TokenStream) -> String {
    match syn::parse2::<ThemeSchema>(input) {
        Ok(_) => panic!("schema unexpectedly parsed"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn generates_typed_contract_from_external_contract_and_values() {
    let code = ThemeCodegen::from_contract(
        fixture("data/contract.tokens"),
        fixture("data/contract-values.toml"),
    )
    .emit_rerun_if_changed(false)
    .generate_string()
    .expect("generated code");

    assert!(code.contains("pub struct ContractFileTheme"));
    assert!(code.contains("pub struct FileButtonTokens"));
    assert!(code.contains("file_button"));
    assert!(code.contains("ContractFileThemeButtonStates"));
    assert!(code.contains("TomlThemeSource"));
    assert!(code.contains("try_load"));
    assert!(code.contains("try_load_with_seed"));
    assert!(code.contains("try_set_seed"));
}

#[test]
fn expand_schema_supports_stateless_component_instances() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct ComponentTheme {
            component ButtonTokens {
                fg: spectrum_theme::Color,
                bg: spectrum_theme::Color,
            }

            button: ButtonTokens,
            toolbar {
                primary: ButtonTokens,
            }
        }
    ))
    .expect("valid schema");

    let code = expand_schema(schema, &facade()).to_string();

    assert!(code.contains("source :: TokenSource"));
    assert!(code.contains("source :: ThemeValue"));
    assert!(code.contains("pub button : ButtonTokens"));
    assert!(code.contains("\"button.fg\""));
    assert!(code.contains("\"toolbar.primary.bg\""));
}

#[test]
fn expand_schema_expands_state_set_aliases() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct TestTheme {
            component ButtonTokens {
                fg: spectrum_theme::Color,
                bg: spectrum_theme::Color,
            }

            states primary_button: ButtonTokens {
                normal,
                hover extends normal,
            }
            states secondary_button inherit primary_button,
            states tertiary_button inherit secondary_button,
        }
    ))
    .expect("valid schema");

    let code = expand_schema(schema, &facade()).to_string();

    assert!(code.contains("pub primary_button : TestThemePrimaryButtonStates"));
    assert!(code.contains("pub secondary_button : TestThemeSecondaryButtonStates"));
    assert!(code.contains("pub tertiary_button : TestThemeTertiaryButtonStates"));
    assert!(code.contains("pub enum TestThemeSecondaryButtonState"));
    assert!(code.contains("\"secondary_button.hover.fg\""));
    assert!(code.contains("\"secondary_button.normal.fg\""));
    assert!(code.contains("\"tertiary_button.hover.fg\""));
}

#[test]
fn writes_generated_contract_to_requested_path() {
    let dir = std::env::temp_dir().join(format!("spectrum-codegen-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("temp dir");
    let output = dir.join("tokens.rs");

    let path = ThemeCodegen::from_contract(
        fixture("data/contract.tokens"),
        fixture("data/contract-values.toml"),
    )
    .emit_rerun_if_changed(false)
    .generate_to(&output)
    .expect("generated file");

    assert_eq!(path, output);
    assert!(
        fs::read_to_string(path)
            .expect("generated source")
            .contains("ContractFileTheme")
    );
}

#[test]
fn rejects_invalid_external_contracts() {
    let error = ThemeCodegen::from_contract(
        fixture("ui/invalid_contract.tokens"),
        fixture("data/contract-values.toml"),
    )
    .emit_rerun_if_changed(false)
    .generate_string()
    .expect_err("invalid contract");

    assert!(matches!(error, CodegenError::InvalidContract(_)));
}

#[test]
fn expand_schema_passes_attributes_to_top_level_struct() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        #[derive(Clone)]
        pub struct TestTheme {
            surface {
                background: spectrum_theme::Color,
            }
        }
    ))
    .expect("valid schema");

    let tokens = expand_schema(schema, &facade());
    let code = tokens.to_string();

    // Attribute must appear before the top-level struct.
    let struct_pos = code.find("pub struct TestTheme").expect("top-level struct");
    let derive_pos = code[..struct_pos]
        .rfind("# [derive (Clone)]")
        .expect("derive on top-level struct");
    assert!(derive_pos < struct_pos);
}

#[test]
fn expand_schema_cascades_attributes_to_generated_sub_structs() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        #[derive(Clone)]
        pub struct TestTheme {
            surface {
                background: spectrum_theme::Color,
            }
        }
    ))
    .expect("valid schema");

    let tokens = expand_schema(schema, &facade());
    let code = tokens.to_string();

    // The generated sub-struct is named TestThemeSurface (root + PascalCase path).
    let sub_pos = code
        .find("pub struct TestThemeSurface")
        .expect("sub-struct");
    let derive_pos = code[..sub_pos]
        .rfind("# [derive (Clone)]")
        .expect("derive on sub-struct");
    assert!(derive_pos < sub_pos);
}

#[test]
fn expand_schema_without_attributes_produces_no_user_derives() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct TestTheme {
            surface {
                background: spectrum_theme::Color,
            }
        }
    ))
    .expect("valid schema");

    let tokens = expand_schema(schema, &facade());
    let code = tokens.to_string();

    // No user derive should appear anywhere (only internal allow/doc attributes).
    assert!(!code.contains("#[derive("));
}

#[test]
fn rejects_unknown_state_parent() {
    let error = parse_schema_error(quote::quote!(
        pub struct TestTheme {
            component ButtonTokens {
                fg: spectrum_theme::Color,
            }
            states button: ButtonTokens {
                normal,
                hover extends missing,
            }
        }
    ));

    assert!(error.contains("unknown parent state `missing`"));
}

#[test]
fn rejects_state_inheritance_cycles() {
    let error = parse_schema_error(quote::quote!(
        pub struct TestTheme {
            component ButtonTokens {
                fg: spectrum_theme::Color,
            }
            states button: ButtonTokens {
                normal extends hover,
                hover extends normal,
            }
        }
    ));

    assert!(error.contains("state inheritance cycle"));
}

#[test]
fn rejects_duplicate_state_names() {
    let error = parse_schema_error(quote::quote!(
        pub struct TestTheme {
            component ButtonTokens {
                fg: spectrum_theme::Color,
            }
            states button: ButtonTokens {
                normal,
                normal,
            }
        }
    ));

    assert!(error.contains("duplicate state `normal`"));
}

#[test]
fn expand_schema_reports_unknown_state_components() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct TestTheme {
            states button: ButtonTokens {
                normal,
            }
        }
    ))
    .expect("schema parses");

    let code = expand_schema(schema, &facade()).to_string();

    assert!(code.contains("compile_error"));
    assert!(code.contains("unknown state component `ButtonTokens`"));
}

#[test]
fn expand_schema_reports_unknown_state_alias_targets() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct TestTheme {
            states secondary_button inherit primary_button,
        }
    ))
    .expect("schema parses");

    let code = expand_schema(schema, &facade()).to_string();

    assert!(code.contains("compile_error"));
    assert!(code.contains("unknown inherited state set `primary_button`"));
}

#[test]
fn expand_schema_reports_state_alias_cycles() {
    let schema: ThemeSchema = syn::parse2(quote::quote!(
        pub struct TestTheme {
            states primary_button inherit secondary_button,
            states secondary_button inherit primary_button,
        }
    ))
    .expect("schema parses");

    let code = expand_schema(schema, &facade()).to_string();

    assert!(code.contains("compile_error"));
    assert!(code.contains("state set inheritance cycle"));
}
