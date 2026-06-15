# Void Spectrum

Void Spectrum is a workspace for a typed theme-token engine written in Rust.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `spectrum-core` | Platform-independent values and resolved theme types |
| `spectrum-schema` | Serializable theme specification and configuration formats |
| `spectrum-palette` | Seed-color and tonal-palette generation boundary |
| `spectrum-resolver` | Merge, reference resolution, validation, and error reporting |
| `spectrum-macros` | Compile-time typed token generation |
| `spectrum-export` | Platform-neutral export infrastructure |
| `spectrum-theme` | Public facade and feature coordination |

## Theme Macros

`define_theme_tokens!` defines a typed contract that can be populated from a
theme loaded at runtime. `include_theme_tokens!` reads and embeds a static TOML
theme at compile time:

```rust
include_theme_tokens! {
    pub struct AppTheme;
    source = "themes/app.toml";
    format = toml;
}
```

> **Known rust-analyzer false positive:** rust-analyzer may report
> `No such file or directory (os error 2)` on the `source` string even when the
> path is valid. The path is resolved relative to the Rust source file that
> invokes the macro. Use `cargo check` or `cargo build` as the authoritative
> result; this editor diagnostic does not prevent the theme from being embedded.

## Development

```bash
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --no-deps
```

See [Architecture](docs/architecture.md) for dependency direction and
[Dependencies](docs/dependencies.md) for the initial dependency assessment.
