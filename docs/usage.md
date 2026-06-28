# Void Spectrum — Usage Guide

## Quick Start

```toml
# Cargo.toml
[dependencies]
spectrum-theme = "0.1"
```

The simplest way—write a few lines of DSL to get a type-safe theme struct:

```rust
use spectrum_theme::{define_theme_tokens, Color};

define_theme_tokens! {
    pub struct MiniTheme {
        surface {
            background: Color,
            foreground: Color,
        }
    }
}

// Expands to:
// pub struct MiniTheme { pub surface: MiniThemeSurface }
// pub struct MiniThemeSurface { pub background: Color, pub foreground: Color }
// impl MiniTheme { fn try_from_source(...) fn reload(...) }
```

---

## Two Paths

| | `define_theme_tokens!` | `build.rs` + `ThemeCodegen` |
|---|---|---|
| Token structure source | Hand-written DSL | Auto-derived from TOML |
| Theme data | Provided at runtime | Embedded at compile time |
| Requires external file | No | Yes (TOML) |
| IDE completion | ✅ via macro expansion | ✅ via `include!` of real file |
| Material colors | ✅ with manual seed | ✅ declared in TOML |
| `try_load` / `try_set_seed` | ❌ | ✅ |

---

## Path 1: Inline DSL

### Supported Token Types

```rust
use spectrum_theme::{define_theme_tokens, Color, Length, Radius, FontWeight, FontStyle, LineHeight, ShadowLayer};

define_theme_tokens! {
    pub struct FullTheme {
        colors {
            primary: Color,
        }
        spacing {
            gutter: Length,
        }
        corners {
            card: Radius,
        }
        typography {
            weight: FontWeight,
            style: FontStyle,
            leading: LineHeight,
        }
        effects {
            elevation: ShadowLayer,
        }
    }
}
```

### User Attributes

Outer attributes placed before `struct` are applied to **every** generated struct
(the root struct and all nested sub-structs):

```rust
use spectrum_theme::{define_theme_tokens, Color, Radius};

define_theme_tokens! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Themed {
        surface {
            background: Color,
        }
        corners {
            card: Radius,
        }
    }
}
// Themed, ThemedSurface, and ThemedCorners all get #[derive(Clone, Debug, PartialEq, Eq)].
```

Multiple `#[derive]` attributes and split attributes are supported:

```rust
define_theme_tokens! {
    #[derive(Clone)]
    #[derive(Debug)]
    pub struct SplitAttrTheme {
        surface {
            background: Color,
        }
    }
}
```

### Reusable Component State Sets

Use `component` when several UI states share the same internal fields. The
component is generated once, and every state field uses that same Rust type:

```rust
use spectrum_theme::{define_theme_tokens, Color, Radius};

define_theme_tokens! {
    #[derive(Clone, Debug, PartialEq)]
    pub struct AppTheme {
        component ButtonTokens {
            fg: Color,
            bg: Color,
            border: Color,
            radius: Radius,
        }

        states button: ButtonTokens {
            normal,
            hover extends normal,
            press_down extends hover,
            focus extends normal,
        }
    }
}
```

This generates:

```rust
pub struct ButtonTokens {
    pub fg: Color,
    pub bg: Color,
    pub border: Color,
    pub radius: Radius,
}

pub struct AppThemeButtonStates {
    pub normal: ButtonTokens,
    pub hover: ButtonTokens,
    pub press_down: ButtonTokens,
    pub focus: ButtonTokens,
}

pub enum AppThemeButtonState {
    Normal,
    Hover,
    PressDown,
    Focus,
}
```

State lookup and declared state relationships are available at runtime:

```rust
let hover = theme.button.get(AppThemeButtonState::Hover);
let parent = AppThemeButtonState::PressDown.parent();

assert_eq!(parent, Some(AppThemeButtonState::Hover));
```

The generated token paths are still explicit:

```text
button.normal.fg
button.hover.fg
button.press_down.fg
button.focus.fg
```

`extends` records the state relationship for UI code. It does not currently
fill missing token values from the parent state; each generated token path must
be present in the source.

### Building an Instance at Runtime

```rust
use spectrum_theme::__private::*;
use std::convert::Infallible;

// ① Implement a TokenSource
struct MySource;

impl TokenSource for MySource { type Error = Infallible; }
impl ColorSource for MySource {
    fn color(&self, _: &str) -> Result<Color, Self::Error> {
        Ok(Color::new(30, 30, 46))
    }
}
impl LengthSource for MySource {
    fn length(&self, _: &str) -> Result<Length, Self::Error> {
        Ok("8px".parse().unwrap())
    }
}
impl RadiusSource for MySource {
    fn radius(&self, _: &str) -> Result<Radius, Self::Error> {
        Ok("12px".parse().unwrap())
    }
}
impl FontWeightSource for MySource {
    fn font_weight(&self, _: &str) -> Result<FontWeight, Self::Error> {
        Ok(FontWeight::new(450).unwrap())
    }
}
impl FontStyleSource for MySource {
    fn font_style(&self, _: &str) -> Result<FontStyle, Self::Error> {
        Ok(FontStyle::Normal)
    }
}
impl LineHeightSource for MySource {
    fn line_height(&self, _: &str) -> Result<LineHeight, Self::Error> {
        Ok("1.5".parse().unwrap())
    }
}
impl ShadowSource for MySource {
    fn shadow(&self, _: &str) -> Result<ShadowLayer, Self::Error> {
        let px = |v| Length::new(v, LengthUnit::Px).unwrap();
        ShadowLayer::new(Color::new(0,0,0), px(0.0), px(2.0), px(8.0), px(0.0)).unwrap()
    }
}

// ② Build
let theme = FullTheme::try_from_source(&MySource).unwrap();
```

### Using ResolvedTheme

`ResolvedTheme` is the output of the resolver and already implements all `*Source` traits:

```rust
use spectrum_schema::ThemeSpec;
use spectrum_resolver::resolve_theme;

// Runtime: parse TOML, resolve references, derive Material colors
let spec: ThemeSpec = toml::from_str(&fs::read_to_string("theme.toml")?).unwrap();
let resolved = resolve_theme(&spec).unwrap();

// Pass directly—ResolvedTheme is a TokenSource
let theme = FullTheme::try_from_source(&resolved).unwrap();
```

### Switching Themes at Runtime (reload)

```rust
// Load theme A
let mut theme = FullTheme::try_from_source(&resolved_a).unwrap();

// Switch to theme B in-place—no reallocation
theme.reload(&resolved_b).unwrap();
```

---

## Path 2: TOML + build.rs

### Step 1: Create `theme.toml`

Place it in your project root or `src/`:

```toml
[meta]
name = "Dawn"
mode = "light"
author = "Alice"
version = "1.0"
description = "A warm light theme"

seed = "#6750a4"

# ── Colors (direct, reference, or Material role) ──
[colors]
"surface.background" = "#fef7ff"
"surface.foreground" = "#1d1b20"
"accent.primary" = "{material.primary}"
"accent.on_primary" = "{material.on_primary}"
"status.success" = "#2e7d32"
"border.default" = "{surface.foreground}"          # reference to another token

# ── Lengths ──
[lengths]
"spacing.xs" = "4px"
"spacing.sm" = "8px"
"spacing.md" = "16px"
"editor.line_height" = "1.5"

# ── Radii ──
[radii]
"radius.sm" = "4px"
"radius.md" = "8px"
"radius.full" = "9999px"

# ── Font weights ──
[font_weights]
"font.body" = "400"
"font.heading" = "700"

# ── Font styles ──
[font_styles]
"font.body" = "normal"
"font.code" = "italic"

# ── Line heights ──
[line_heights]
"line_height.body" = "1.5"
"line_height.heading" = "1.2"

# ── Shadows ──
[[shadows]]
path = "shadow.sm"
color = "#00000026"
offset_x = "0px"
offset_y = "1px"
blur = "2px"
spread = "0px"

[[shadows]]
path = "shadow.md"
color = "#00000033"
offset_x = "0px"
offset_y = "4px"
blur = "8px"
spread = "0px"
```

### Step 2: Create `build.rs`

```rust
// build.rs
use spectrum_codegen::ThemeCodegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ThemeCodegen::new("theme.toml", "AppTheme").generate()?;
    Ok(())
}
```

### Step 3: Configure `Cargo.toml`

```toml
[build-dependencies]
spectrum-codegen = "0.1"

[dependencies]
spectrum-theme = "0.1"
```

### Step 4: Include in Source

```rust
// src/main.rs
use spectrum_theme::Color;

include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));
// ↑ Generated at compile time by build.rs. Fully indexed by rust-analyzer.

fn main() {
    // Use the embedded defaults
    let theme = AppTheme::try_load().unwrap();

    // Override the brand color at runtime—Material colors recalculate
    let red = AppTheme::try_load_with_seed(Color::new(255, 0, 0)).unwrap();

    // In-place brand color override—only Material fields update
    let mut theme = AppTheme::try_load().unwrap();
    theme.try_set_seed(Color::new(255, 0, 0)).unwrap();
}
```

### Multiple Theme Files

```rust
// build.rs
ThemeCodegen::new("themes/dark.toml", "DarkTheme")
    .output_file("dark_tokens.rs")
    .generate()?;
ThemeCodegen::new("themes/light.toml", "LightTheme")
    .output_file("light_tokens.rs")
    .generate()?;
```

```rust
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/dark_tokens.rs"));
include!(concat!(env!("OUT_DIR"), "/light_tokens.rs"));

let theme = match user_preference {
    "dark" => DarkTheme::try_load().unwrap(),
    _ => LightTheme::try_load().unwrap(),
};
```

### Full ThemeCodegen Configuration

```rust
ThemeCodegen::new("src/theme.toml", "MyTheme")
    .visibility("pub(crate)")          // default "pub"
    .output_file("my_theme_tokens.rs") // default "theme_tokens.rs"
    .facade_path("::spectrum_theme")   // default, rarely changed
    .generate()?;
```

---

## Runtime API Reference

Every generated struct has these methods:

| Method | Signature | Available |
|---|---|---|
| `try_from_source` | `fn try_from_source<S: TokenSource>(source: &S) -> Result<Self, S::Error>` | Always |
| `reload` | `fn reload<S: TokenSource>(&mut self, source: &S) -> Result<(), S::Error>` | Always |
| `try_load` | `fn try_load() -> Result<Self, ThemeBuildError>` | build.rs path |
| `try_load_with_seed` | `fn try_load_with_seed(seed: Color) -> Result<Self, ThemeBuildError>` | build.rs path |
| `try_set_seed` | `fn try_set_seed(&mut self, seed: Color) -> Result<(), ThemeBuildError>` | build.rs path |

```rust
// ─── Inline DSL path (no embedded data) ───
let mut theme = AppTheme::try_from_source(&resolved).unwrap();
theme.reload(&new_resolved).unwrap();

// ─── build.rs path (has embedded data) ───
let theme = AppTheme::try_load().unwrap();                                // embedded defaults
let red = AppTheme::try_load_with_seed(Color::new(255,0,0)).unwrap();     // new brand color
let mut theme = AppTheme::try_load().unwrap();
theme.try_set_seed(Color::new(255,0,0)).unwrap();                         // in-place brand color update
theme.reload(&other_resolved).unwrap();                                    // full theme swap
```

---

## TOML Reference

### Metadata `[meta]`

| Field | Type | Required |
|---|---|---|
| `name` | string | ✅ |
| `mode` | `"dark"` or `"light"` | No, defaults to `"dark"` |
| `author` | string | No |
| `version` | string | No |
| `description` | string | No |

### Seed Color `seed`

Optional. Required for Material role references (e.g., `{material.primary}`) to resolve.

```toml
seed = "#6750a4"       # RGB
seed = "#6750a480"     # RGBA
```

### `[colors]` — Color Tokens

Values can be:

```toml
# Direct color
"surface.bg" = "#1e1e2e"

# Reference to another token in the same file
"border.focus" = "{accent.primary}"

# Material role
"accent.primary" = "{material.primary}"
```

State-set tokens use the same flat paths as the generated contract:

```toml
[colors]
"button.normal.fg" = "#ffffff"
"button.normal.bg" = "{material.primary}"
"button.normal.border" = "{material.outline}"
"button.hover.fg" = "#ffffff"
"button.hover.bg" = "{material.primary_container}"
"button.hover.border" = "{material.primary}"
"button.press_down.fg" = "#ffffff"
"button.press_down.bg" = "#4f378b"
"button.press_down.border" = "{material.primary}"
"button.focus.fg" = "#ffffff"
"button.focus.bg" = "{material.primary}"
"button.focus.border" = "{material.primary}"

[radii]
"button.normal.radius" = "8px"
"button.hover.radius" = "8px"
"button.press_down.radius" = "8px"
"button.focus.radius" = "8px"
```

This format is intentionally explicit in the current schema. A future
contract-aware configuration format can reduce this repetition by applying
state inheritance while loading.

### Material Color Roles

| Role | Purpose |
|---|---|
| `material.primary` | Primary brand color |
| `material.on_primary` | Content on primary |
| `material.primary_container` | Primary container |
| `material.on_primary_container` | Content on primary container |
| `material.secondary` | Secondary color |
| `material.on_secondary` | Content on secondary |
| `material.secondary_container` | Secondary container |
| `material.on_secondary_container` | Content on secondary container |
| `material.tertiary` | Tertiary color |
| `material.on_tertiary` | Content on tertiary |
| `material.tertiary_container` | Tertiary container |
| `material.on_tertiary_container` | Content on tertiary container |
| `material.background` | Background |
| `material.on_background` | Content on background |
| `material.surface` | Surface |
| `material.on_surface` | Content on surface |
| `material.surface_dim` | Dim surface |
| `material.surface_bright` | Bright surface |
| `material.surface_container_lowest` | Lowest container |
| `material.surface_container_low` | Low container |
| `material.surface_container` | Default container |
| `material.surface_container_high` | High container |
| `material.surface_container_highest` | Highest container |
| `material.surface_variant` | Surface variant |
| `material.on_surface_variant` | Content on surface variant |
| `material.inverse_surface` | Inverse surface |
| `material.inverse_on_surface` | Content on inverse surface |
| `material.outline` | Outline |
| `material.outline_variant` | Outline variant |
| `material.shadow` | Shadow |
| `material.scrim` | Scrim |
| `material.surface_tint` | Surface tint |
| `material.inverse_primary` | Inverse primary |
| `material.primary_fixed` | Fixed primary |
| `material.primary_fixed_dim` | Dim fixed primary |
| `material.on_primary_fixed` | Content on fixed primary |
| `material.on_primary_fixed_variant` | Content on fixed primary variant |
| `material.secondary_fixed` | Fixed secondary |
| `material.secondary_fixed_dim` | Dim fixed secondary |
| `material.on_secondary_fixed` | Content on fixed secondary |
| `material.on_secondary_fixed_variant` | Content on fixed secondary variant |
| `material.tertiary_fixed` | Fixed tertiary |
| `material.tertiary_fixed_dim` | Dim fixed tertiary |
| `material.on_tertiary_fixed` | Content on fixed tertiary |
| `material.on_tertiary_fixed_variant` | Content on fixed tertiary variant |
| `material.error` | Error |
| `material.on_error` | Content on error |
| `material.error_container` | Error container |
| `material.on_error_container` | Content on error container |

### `[lengths]` — Length Tokens

Supports `px`, `rem`, `em`:

```toml
[lengths]
"spacing.sm" = "4px"
"spacing.lg" = "2rem"
"editor.gutter" = "3em"
```

### `[radii]` — Radius Tokens

```toml
[radii]
"radius.sm" = "4px"
"radius.full" = "9999px"
```

### `[font_weights]` — Font Weight Tokens

```toml
[font_weights]
"font.body" = "400"
"font.bold" = "700"
```

### `[font_styles]` — Font Style Tokens

Supports `"normal"`, `"italic"`, `"oblique"`:

```toml
[font_styles]
"font.body" = "normal"
"font.code" = "italic"
```

### `[line_heights]` — Line Height Tokens

Supports bare numbers (unitless), `px`, `rem`:

```toml
[line_heights]
"line_height.body" = "1.5"
"line_height.code" = "20px"
```

### `[[shadows]]` — Shadow Tokens

```toml
[[shadows]]
path = "shadow.card"
color = "#00000033"
offset_x = "0px"
offset_y = "4px"
blur = "8px"
spread = "0px"
```

---

## Custom Type Extension

Built-in types (`Color`, `Length`, etc.) are not closed. You can add your own token types:

```rust
use spectrum_theme::__private::{TokenSource, TokenValue};

// ① Define your type
#[derive(Debug, Clone, Copy)]
pub struct Padding(pub u16);

// ② Declare the Source capability it needs
pub trait PaddingSource: TokenSource {
    fn padding(&self, path: &str) -> Result<Padding, Self::Error>;
}

// ③ Teach the system how to read it from a Source
impl<S: PaddingSource> TokenValue<S> for Padding {
    fn read(source: &S, path: &str) -> Result<Self, S::Error> {
        source.padding(path)
    }
}

// ④ Use it in the DSL
define_theme_tokens! {
    pub struct CustomTheme {
        spacing {
            pad: Padding,  // ← custom type
        }
    }
}

// ⑤ Your Source implements both standard and custom traits
struct MySource;

impl TokenSource for MySource { type Error = Infallible; }
impl ColorSource for MySource { ... }
impl PaddingSource for MySource {
    fn padding(&self, _: &str) -> Result<Padding, Self::Error> {
        Ok(Padding(12))
    }
}

let theme = CustomTheme::try_from_source(&MySource).unwrap();
assert_eq!(theme.spacing.pad.0, 12);
```

> **Pattern**: every custom type needs a pair—a `XxxSource` trait (declares that a Source can provide this value) + `impl TokenValue<S> for Xxx` (declares how the type reads from a Source). This is the same pattern as the built-in `ColorSource` + `impl TokenValue<S> for Color`.

---

## Feature Flags

```toml
[dependencies]
spectrum-theme = { version = "0.1", features = ["macros", "seed", "serde"] }
```

| Feature | Purpose |
|---|---|
| `macros` | Enables the `define_theme_tokens!` macro |
| `seed` | Enables Material color derivation (`material.primary`, etc.) |
| `serde` | Enables serde serialization for core types |
| `json` | Enables JSON-format theme file loading (schema layer) |
| `toml` | Enables TOML-format theme file loading (schema layer) |
| `export` | Enables export infrastructure (CSS, Design Tokens JSON, etc.) |
