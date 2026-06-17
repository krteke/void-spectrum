# Dependency Assessment

The initial dependency set is deliberately small and centralized in the root
workspace manifest.

| Dependency | Intended use | Decision |
| --- | --- | --- |
| `serde` | Schema and optional core serialization | Include |
| `serde_json` | JSON theme configuration | Optional feature |
| `toml` | TOML theme configuration | Optional feature |
| `thiserror` | Stable resolver error categories and sources | Include |
| `proc-macro2` | Procedural macro token model | Include in macro crate |
| `quote` | Procedural macro output | Include in macro crate |
| `syn` | Procedural macro input parsing | Include in macro crate |
| `palette` | Color-space conversion primitives | Optional feature |
| `material-colors` | Material color roles from a Seed color | Optional `seed` feature |
| `ratatui` | Terminal UI adapter target | Adapter crate dependency |
| `iced_core` | Formal Iced conversion target | Adapter crate dependency |
| `iced` | Visual Iced example runtime | Optional adapter feature |

## Seed color algorithm

The `palette` crate remains available for general color-space conversions. The
Seed-to-Material-role path currently uses `material-colors` behind the optional
`seed` feature. Any replacement must satisfy:

1. Algorithm compatibility tests against published Material reference values.
2. A Rust 1.88 compatible release.
3. Acceptable maintenance, license, and dependency footprint.
4. Deterministic output suitable for snapshot and contract tests.

## Deferred dependencies

egui, syntect, CSS helper dependencies, and Design Tokens export dependencies
are deferred until their adapter or export contracts are implemented.
