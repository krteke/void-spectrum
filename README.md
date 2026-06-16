# Void Spectrum

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
| `spectrum-export` | Platform-neutral export infrastructure |
| `spectrum-theme` | Public facade and feature coordination |

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

See [Architecture](docs/architecture.md) for dependency direction and
[Dependencies](docs/dependencies.md) for the initial dependency assessment.
