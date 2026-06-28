# Workspace Architecture

## Dependency direction

```text
spectrum-core
    ^
    |
spectrum-schema     spectrum-palette
    ^                    ^
    |                    |
    +--- spectrum-resolver
              ^
              |
        spectrum-codegen
              |
              v
        build.rs output ---> spectrum-theme

spectrum-codegen <- spectrum-macros -------------------> spectrum-theme
spectrum-core <- spectrum-ratatui <- spectrum-theme
spectrum-core <- spectrum-iced ----^
```

The dependency graph must remain acyclic. `spectrum-core` is the lowest-level
crate and must not depend on configuration formats, macro parsing, or rendering
frameworks.

## Crate boundaries

### `spectrum-core`

Owns platform-independent value types and the fully resolved, strongly typed
theme representation. It may expose optional serialization support, but it
must not parse theme files.

### `spectrum-schema`

Owns partial configuration structures, token references, theme metadata, and
format-specific loading features. Schema values may be incomplete because the
resolver supplies defaults and derived values.

### `spectrum-palette`

Owns the Seed Color to tonal palette boundary. The `seed` feature uses
`material-colors` to produce Material-style roles while keeping the algorithm
dependency outside `spectrum-core`.

### `spectrum-resolver`

Owns precedence, merging, reference expansion, cycle detection, type checking,
and contract validation. Its output is a complete `spectrum-core` theme.

### `spectrum-codegen`

Owns build-time Rust source generation for typed token contracts. It parses and
resolves static theme files in build scripts and emits ordinary Rust source that
consuming crates include from `OUT_DIR`, keeping IDE diagnostics and completion
aware of generated token fields.

Inline contracts can also define reusable component structs and state sets. A
component such as `ButtonTokens` is generated once, while a state set such as
`button` generates `normal`, `hover`, and `focus` fields that all share the
same component type. State sets keep UI state relationships in the generated
Rust contract without forcing every state path to become a distinct nested
struct type. Declared `extends` relationships also drive generated source
lookup fallback for missing state fields.

The resolver still stores resolved values as flat token paths. Codegen maps
those paths into reusable Rust structs and applies state fallback while building
the typed theme; it does not require platform adapters or the resolver to know
about buttons, focus states, or animation semantics.

### `spectrum-macros`

Owns procedural macros for inline typed token contracts. File-driven generation
belongs in `spectrum-codegen` so macro expansion does not hide file contents
from rust-analyzer.

### `spectrum-theme`

Provides the user-facing facade and coordinates optional features. It should
contain little behavior of its own.

### `spectrum-ratatui`

Owns Ratatui conversions for core values. It exposes adapter traits and keeps
terminal-specific choices out of `spectrum-core`.

### `spectrum-iced`

Owns Iced conversions for core values. The crate depends on `iced_core` for
formal conversions and gates the full visual example behind its `runtime`
feature.

## Deferred adapters and exports

CSS, Design Tokens JSON, egui, syntect, and other platform outputs are not part
of `0.1.0`. They should be added as focused crates or features only after their
conversion contracts are concrete.
