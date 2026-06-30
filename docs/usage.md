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
| Token structure source | Hand-written DSL | External contract DSL |
| Theme data | Provided at runtime | Embedded at compile time |
| Requires external file | No | Yes (contract + TOML) |
| IDE completion | ✅ via macro expansion | ✅ via `include!` of real file |
| Material colors | ✅ with manual seed | ✅ declared in TOML |
| `try_load` | ❌ | ✅ |

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

### Reusable Components and State Sets

Use `component` once to define a reusable token struct. It can be instantiated
directly for stateless tokens, or through `states` when several UI states share
the same internal fields:

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

        button: ButtonTokens,

        states nav_button: ButtonTokens {
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

pub struct AppThemeNavButtonStates {
    pub normal: ButtonTokens,
    pub hover: ButtonTokens,
    pub press_down: ButtonTokens,
    pub focus: ButtonTokens,
}

pub enum AppThemeNavButtonState {
    Normal,
    Hover,
    PressDown,
    Focus,
}
```

`theme.button` is a plain `ButtonTokens` for stateless elements. The state
fields, such as `theme.nav_button.normal` and `theme.nav_button.hover`, are also
`ButtonTokens`.

State lookup and declared state relationships are available at runtime:

```rust
let hover = theme.nav_button.get(AppThemeNavButtonState::Hover);
let parent = AppThemeNavButtonState::PressDown.parent();

assert_eq!(parent, Some(AppThemeNavButtonState::Hover));
```

The generated token paths are still explicit:

```text
button.fg
nav_button.normal.fg
nav_button.hover.fg
nav_button.press_down.fg
nav_button.focus.fg
```

`extends` records the state relationship for UI code and controls source lookup
fallback. If a token is missing from `press_down`, the generated reader tries
`hover`, then `normal`. Non-missing source errors are returned immediately and
do not fall back. The parent state must be declared in the same state set;
duplicate state names and inheritance cycles are rejected by the contract
parser.

### Building an Instance at Runtime

```rust
use spectrum_theme::source::{ThemeValue, TokenSource};
use std::convert::Infallible;

// ① Implement a TokenSource
struct MySource;

impl TokenSource for MySource { type Error = Infallible; }
impl ThemeValue<MySource> for Color {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok(Color::new(30, 30, 46))
    }
}
impl ThemeValue<MySource> for Length {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok("8px".parse().unwrap())
    }
}
impl ThemeValue<MySource> for Radius {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok("12px".parse().unwrap())
    }
}
impl ThemeValue<MySource> for FontWeight {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok(FontWeight::new(450).unwrap())
    }
}
impl ThemeValue<MySource> for FontStyle {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok(FontStyle::Normal)
    }
}
impl ThemeValue<MySource> for LineHeight {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok("1.5".parse().unwrap())
    }
}
impl ThemeValue<MySource> for ShadowLayer {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        let px = |v| Length::new(v, LengthUnit::Px).unwrap();
        Ok(ShadowLayer::new(Color::new(0,0,0), px(0.0), px(2.0), px(8.0), px(0.0)).unwrap())
    }
}

// ② Build
let theme = FullTheme::try_from_source(&MySource).unwrap();
```

### Contract-Aware TOML Source

With the `toml` feature, a generated contract can read TOML tables directly.
The contract supplies field types, so the file does not need separate
`[colors]`, `[lengths]`, or `[radii]` buckets:

```rust
use spectrum_theme::config::TomlThemeSource;

let source: TomlThemeSource = r##"
seed = "#6750a4"

[meta]
mode = "light"

[button.normal]
fg = "{material.primary}"
bg = "{material.surface}"
radius = "8px"

[button.hover]
bg = "{material.primary_container}"
"##
.parse()
.unwrap();

let theme = AppTheme::try_from_source(&source).unwrap();
```

For a state set such as `hover extends normal`, missing fields under
`[button.hover]` fall back through the declared parent chain. Scalar values are
parsed according to the generated field type. Same-type token references such
as `fg = "{button.normal.fg}"` are supported; color values also support
`{material.*}` roles when a seed is present.

Shadow fields use a table at the token path:

```toml
[shadow.card]
color = "#00000080"
offset_x = "0px"
offset_y = "2px"
blur = "8px"
spread = "0px"
```

Custom types can implement `ThemeValue<TomlThemeSource>` and read their raw
scalar text through `source.token_text(path)`.

### Switching Themes at Runtime (reload)

```rust
// Load theme A
let mut theme = FullTheme::try_from_source(&source_a).unwrap();

// Switch to theme B in-place—no reallocation
theme.reload(&source_b).unwrap();
```

---

## Path 2: External Contract + TOML + build.rs

Use this path when you want rust-analyzer-visible generated Rust code while
keeping the same contract grammar as `define_theme_tokens!`.

### Step 1: Create `theme.tokens`

```rust
pub struct AppTheme {
    component ButtonTokens {
        fg: spectrum_theme::Color,
        bg: spectrum_theme::Color,
        radius: spectrum_theme::Radius,
    }

    button: ButtonTokens,

    states button_state: ButtonTokens {
        normal,
        hover extends normal,
        press_down extends hover,
    }
}
```

### Step 2: Create `theme.toml`

```toml
seed = "#6750a4"

[meta]
mode = "light"

[button]
fg = "{material.primary}"
bg = "{material.surface}"
radius = "8px"

[button_state.normal]
fg = "{material.primary}"
bg = "{material.surface}"
radius = "8px"

[button_state.hover]
bg = "{material.primary_container}"
```

### Step 3: Create `build.rs`

```rust
use spectrum_codegen::ThemeCodegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ThemeCodegen::from_contract("theme.tokens", "theme.toml").generate()?;
    Ok(())
}
```

### Step 4: Include in Source

```rust
include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));

fn main() {
    let theme = AppTheme::try_load().unwrap();
    let hover = theme.button_state.hover;
}
```

`from_contract` validates the external contract and the TOML syntax at build
time. The generated `try_load` uses `TomlThemeSource`, so the consuming crate
must enable `spectrum-theme`'s `toml` feature.

### Multiple Theme Files

```rust
// build.rs
ThemeCodegen::from_contract("themes/app.tokens", "themes/dark.toml")
    .output_file("dark_tokens.rs")
    .generate()?;
ThemeCodegen::from_contract("themes/app.tokens", "themes/light.toml")
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
ThemeCodegen::from_contract("src/theme.tokens", "src/theme.toml")
    .output_file("theme_tokens.rs")
    .facade_path("::spectrum_theme")
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

`try_from_source` and `reload` also require every token value type in the
contract to implement `ThemeValue<S>` for the provided source.

```rust
// ─── Inline DSL path (no embedded data) ───
let mut theme = AppTheme::try_from_source(&source).unwrap();
theme.reload(&new_source).unwrap();

// ─── build.rs path (has embedded data) ───
let mut theme = AppTheme::try_load().unwrap();                             // embedded defaults
theme.reload(&other_source).unwrap();                                       // full theme swap
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

Runtime code can override the TOML seed without changing token values:

```rust
let source = TomlThemeSource::parse(include_str!("theme.toml"))?
    .with_seed(Color::new(0, 0, 255));
let theme = AppTheme::try_from_source(&source)?;
```

Themes generated from external contract files also expose seed-specific loaders:

```rust
let mut theme = AppTheme::try_load_with_seed(Color::new(0, 0, 255))?;
theme.try_set_seed(Color::new(255, 0, 0))?;
```

### Contract Token Tables

Token paths come from the generated contract. Nested contract fields map to
TOML tables, and the final field name is the scalar key:

```toml
[surface]
bg = "#1e1e2e"

[accent]
primary = "{material.primary}"

[border]
focus = "{accent.primary}"
```

Component instances and state sets use the same rule:

```toml
[button]
fg = "#ffffff"
bg = "{material.primary}"
radius = "8px"

[nav_button.normal]
fg = "#ffffff"
bg = "{material.primary}"
radius = "8px"

[nav_button.hover]
bg = "{material.primary_container}"
```

Missing state fields inherit through the generated `extends` chain, so the
example above reads `nav_button.hover.fg` and `nav_button.hover.radius` from
`nav_button.normal`.

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

### Scalar Values

Built-in values are parsed from scalar strings according to the contract field
type. Custom values can implement `ThemeValue<TomlThemeSource>` and parse the
raw `source.token_text(path)` string.

```toml
[spacing]
sm = "4px"
lg = "2rem"

[font]
body = "400"
style = "italic"

[line_height]
body = "1.5"
code = "20px"
```

### Shadow Tokens

```toml
[shadow.card]
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
use spectrum_theme::source::{ThemeValue, TokenSource};

// ① Define your type
#[derive(Debug, Clone, Copy)]
pub struct Padding(pub u16);

// ② Use it in the DSL
define_theme_tokens! {
    pub struct CustomTheme {
        spacing {
            pad: Padding,  // ← custom type
        }
    }
}

// ③ Teach one Source how to provide it
struct MySource;

impl TokenSource for MySource { type Error = Infallible; }
impl ThemeValue<MySource> for Padding {
    fn read(_: &MySource, _: &str) -> Result<Self, Infallible> {
        Ok(Padding(12))
    }
}

let theme = CustomTheme::try_from_source(&MySource).unwrap();
assert_eq!(theme.spacing.pad.0, 12);
```

> **Pattern**: every value type is connected to a concrete source with
> `impl ThemeValue<MySource> for MyValue`. The generated code only calls
> `source.token::<MyValue>("path")`.

---

## Feature Flags

```toml
[dependencies]
spectrum-theme = { version = "0.1", features = ["macros", "toml", "seed"] }
```

| Feature | Purpose |
|---|---|
| `macros` | Enables the `define_theme_tokens!` macro |
| `seed` | Enables Material color derivation (`material.primary`, etc.) |
| `serde` | Enables serde serialization for core types |
| `toml` | Enables contract-aware TOML source loading |
| `ratatui` | Enables the Ratatui adapter re-export |
| `iced` | Enables the iced adapter re-export |
