//! Build-time typed theme generation for integration tests.

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    spectrum_codegen::ThemeCodegen::new(manifest_dir.join("tests/data/theme.toml"), "FileTheme")
        .generate()?;
    Ok(())
}
