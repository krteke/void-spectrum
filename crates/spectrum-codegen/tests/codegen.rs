//! Behavior tests for build-time typed theme generation.

use spectrum_codegen::{CodegenError, ThemeCodegen};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../spectrum-theme/tests")
        .join(path)
}

#[test]
fn generates_typed_contract_from_theme_file() {
    let code = ThemeCodegen::new(fixture("data/theme.toml"), "FileTheme")
        .emit_rerun_if_changed(false)
        .generate_string()
        .expect("generated code");

    assert!(code.contains("pub struct FileTheme"));
    assert!(code.contains("try_load"));
    assert!(code.contains("try_load_with_seed"));
    assert!(code.contains("try_set_seed"));
    assert!(code.contains("background"));
}

#[test]
fn writes_generated_contract_to_requested_path() {
    let dir = std::env::temp_dir().join(format!("spectrum-codegen-test-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("temp dir");
    let output = dir.join("tokens.rs");

    let path = ThemeCodegen::new(fixture("data/theme.toml"), "FileTheme")
        .emit_rerun_if_changed(false)
        .generate_to(&output)
        .expect("generated file");

    assert_eq!(path, output);
    assert!(
        fs::read_to_string(path)
            .expect("generated source")
            .contains("FileTheme")
    );
}

#[test]
fn rejects_invalid_schema() {
    let error = ThemeCodegen::new(fixture("ui/invalid_schema.toml"), "InvalidSchema")
        .emit_rerun_if_changed(false)
        .generate_string()
        .expect_err("invalid schema");

    assert!(matches!(error, CodegenError::ParseToml { .. }));
}

#[test]
fn rejects_unresolved_references() {
    let error = ThemeCodegen::new(
        fixture("ui/unresolved_reference.toml"),
        "UnresolvedReference",
    )
    .emit_rerun_if_changed(false)
    .generate_string()
    .expect_err("unresolved reference");

    assert!(matches!(error, CodegenError::Resolve { .. }));
}
