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

## Seed color algorithm

The `palette` crate provides general color-space types and conversions, but it
does not by itself guarantee Material 3 HCT and tonal-palette compatibility.
No Material Color Utilities implementation is selected in this skeleton.
Selection requires:

1. Algorithm compatibility tests against published Material reference values.
2. A Rust 1.85 compatible release.
3. Acceptable maintenance, license, and dependency footprint.
4. Deterministic output suitable for snapshot and contract tests.

## Deferred dependencies

Ratatui, egui, iced, syntect, and CSS helper dependencies are not introduced
until their adapter crates and conversion contracts are implemented.
