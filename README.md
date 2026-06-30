# Void Spectrum

[中文](README_ZH.md)

Void Spectrum is a workspace for a typed theme-token engine written in Rust.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `spectrum-core` | Platform-independent theme value types |
| `spectrum-palette` | Seed-color and tonal-palette generation boundary |
| `spectrum-codegen` | Build-time typed token contract code generation |
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
| `toml` | Contract-aware TOML source loading |
| `ratatui` | `spectrum_theme::ratatui` adapter re-export |
| `iced` | `spectrum_theme::iced` adapter re-export |

## Typed Theme Generation

`define_theme_tokens!` defines a typed contract that can be populated from any
`TokenSource`. For static TOML themes, keep the contract and values in separate
files and generate Rust code in `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    spectrum_codegen::ThemeCodegen::from_contract("theme.tokens", "theme.toml")
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
