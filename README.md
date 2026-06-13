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

## Development

```bash
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --no-deps
```

See [Architecture](docs/architecture.md) for dependency direction and
[Dependencies](docs/dependencies.md) for the initial dependency assessment.
