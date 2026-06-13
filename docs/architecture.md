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
        spectrum-theme

spectrum-core <- spectrum-export <- spectrum-theme
spectrum-macros -------------------> spectrum-theme
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

Owns the Seed Color to tonal palette boundary. The Material 3 HCT algorithm is
not selected yet; this crate isolates that decision from the rest of the
workspace.

### `spectrum-resolver`

Owns precedence, merging, reference expansion, cycle detection, type checking,
and contract validation. Its output is a complete `spectrum-core` theme.

### `spectrum-macros`

Owns procedural macros that generate typed token structures and contract
metadata. Macro syntax is intentionally deferred until the token contract is
specified by tests.

### `spectrum-export`

Owns export-oriented intermediate APIs. Concrete CSS and Design Tokens JSON
output can be implemented here without introducing GUI dependencies.

### `spectrum-theme`

Provides the user-facing facade and coordinates optional features. It should
contain little behavior of its own.

## Deferred adapter crates

Adapters will use focused crate names such as `spectrum-css`,
`spectrum-ratatui`, `spectrum-egui`, `spectrum-iced`, and
`spectrum-syntect`. They are deferred because adding them now would lock
framework versions before adapter contracts exist.
