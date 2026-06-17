# Void Spectrum

[中文](README_ZH.md)

Void Spectrum is a workspace for a typed theme-token engine written in Rust.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `spectrum-core` | Platform-independent values and resolved theme types |
| `spectrum-schema` | Serializable theme specification and configuration formats |
| `spectrum-palette` | Seed-color and tonal-palette generation boundary |
| `spectrum-resolver` | Merge, reference resolution, validation, and error reporting |
| `spectrum-codegen` | Build-time typed token code generation |
| `spectrum-macros` | Inline typed token contract generation |
| `spectrum-ratatui` | Ratatui adapter traits and a visual terminal example |
| `spectrum-iced` | Iced adapter traits and a visual GUI example |
| `spectrum-theme` | Public facade and feature coordination |

## Facade features

`spectrum-theme` enables Seed-based Material color generation by default.
Optional features expose focused APIs:

| Feature | Enables |
| --- | --- |
| `macros` | `define_theme_tokens!` |
| `toml` | TOML schema loading support |
| `json` | JSON schema loading support |
| `ratatui` | `spectrum_theme::ratatui` adapter re-export |
| `iced` | `spectrum_theme::iced` adapter re-export |

## Typed Theme Generation

`define_theme_tokens!` defines a typed contract that can be populated from a
theme loaded at runtime. For static TOML themes, generate Rust code in
`build.rs` so rust-analyzer can inspect the emitted structs and fields through
Cargo's build-script output:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    spectrum_codegen::ThemeCodegen::new("themes/app.toml", "AppTheme")
        .generate()?;
    Ok(())
}
```

Then include the generated source from Rust code:

```rust
include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));
```

## Development

```bash
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --no-deps
```

See [Architecture](docs/architecture.md) for dependency direction,
[Dependencies](docs/dependencies.md) for the initial dependency assessment and [Usage](docs/usage.md) for basic usage.
