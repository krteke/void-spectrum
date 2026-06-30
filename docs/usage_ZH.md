# Void Spectrum — 使用指南

## 快速开始

```toml
# Cargo.toml
[dependencies]
spectrum-theme = "0.2.0"
```

最简单的用法——手写几行 DSL，得到一个类型安全的主题结构体：

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

// 展开后生成：
// pub struct MiniTheme { pub surface: MiniThemeSurface }
// pub struct MiniThemeSurface { pub background: Color, pub foreground: Color }
// impl MiniTheme { fn try_from_source(...) fn reload(...) }
```

---

## 两条路径

| | `define_theme_tokens!` | `build.rs` + `ThemeCodegen` |
|---|---|---|
| 令牌结构来源 | 手写 DSL | 外部 contract DSL |
| 主题数据 | 运行时提供 | 编译期嵌入二进制 |
| 是否需外部文件 | 否 | 需 contract + TOML |
| IDE 补全 | ✅ 宏展开可见 | ✅ `include!` 真实文件 |
| Material 颜色 | ✅ 手动指定 seed | ✅ TOML 中声明 |
| `try_load` | ❌ | ✅ |

---

## 路径一：手写 DSL

### 支持的令牌类型

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

### 用户自定义属性

`struct` 前放置的 outer attribute 会应用到**所有**生成的结构体（根结构体和所有嵌套子结构体）：

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
// Themed、ThemedSurface、ThemedCorners 都会获得 #[derive(Clone, Debug, PartialEq, Eq)]。
```

支持多个 `#[derive]` 和拆分属性：

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

### 可复用组件与状态集合

使用 `component` 定义一次组件令牌结构后，可以直接实例化为无状态 token，也可以通过
`states` 为不同 UI 状态实例化同一个 Rust 类型：

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

        states sidebar_button inherit nav_button,
    }
}
```

它会生成：

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

`theme.button` 是普通 `ButtonTokens`，适合无状态元素。`theme.nav_button.normal`、
`theme.nav_button.hover` 等字段同样是 `ButtonTokens`，适合需要状态关系的元素。

运行时可以通过状态枚举访问，并读取声明的状态关系：

```rust
let hover = theme.nav_button.get(AppThemeNavButtonState::Hover);
let parent = AppThemeNavButtonState::PressDown.parent();

assert_eq!(parent, Some(AppThemeNavButtonState::Hover));
```

生成器读取的令牌路径仍然是显式路径：

```text
button.fg
nav_button.normal.fg
nav_button.hover.fg
nav_button.press_down.fg
nav_button.focus.fg
sidebar_button.normal.fg
sidebar_button.hover.fg
```

`extends` 既记录状态关系，也控制 source 读取时的回退顺序。如果 `press_down` 缺少某个
字段，生成代码会继续尝试 `hover`，再尝试 `normal`。非缺失类错误会直接返回，不会被
父状态掩盖。父状态必须声明在同一个状态集合内；重复状态名和循环继承会被契约解析器拒绝。

`states sidebar_button inherit nav_button` 是契约层面的结构复用。它等价于再声明一个
`states sidebar_button: ButtonTokens`，并复制 `normal`、`hover extends normal`、
`press_down extends hover`、`focus extends normal` 这组状态定义。值仍然从
`sidebar_button.*` 路径读取；继承出来的状态集合不会回退读取 `nav_button.*` 的 token 值。

### 运行时构造实例

```rust
use spectrum_theme::source::{ThemeValue, TokenSource};
use std::convert::Infallible;

// ① 实现一个 TokenSource
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

// ② 构造
let theme = FullTheme::try_from_source(&MySource).unwrap();
```

### Contract-Aware TOML Source

启用 `toml` feature 后，生成契约可以直接读取 TOML 表结构。字段类型来自契约本身，
因此配置文件不需要再拆成 `[colors]`、`[lengths]`、`[radii]` 这类固定分桶：

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

对于 `hover extends normal` 这样的状态集合，`[button.hover]` 中缺失的字段会沿声明的父状态链回退。
标量值会按照生成字段的类型解析。同类型 token 引用，例如 `fg = "{button.normal.fg}"`，
可以正常工作；颜色字段在存在 seed 时还支持 `{material.*}` 角色。

阴影字段使用 token path 对应的表：

```toml
[shadow.card]
color = "#00000080"
offset_x = "0px"
offset_y = "2px"
blur = "8px"
spread = "0px"
```

自定义类型可以实现 `ThemeValue<TomlThemeSource>`，并通过 `source.token_text(path)`
读取原始标量文本。

### 运行时切换主题（reload）

```rust
// 加载主题 A
let mut theme = FullTheme::try_from_source(&source_a).unwrap();

// 运行时切换到主题 B——原地更新，不重建实例
theme.reload(&source_b).unwrap();
```

---

## 路径二：外部 Contract + TOML + build.rs

当你既想使用和 `define_theme_tokens!` 相同的契约语法，又希望生成出来的 Rust 文件能被
rust-analyzer 完整索引时，使用这条路径。

### 第一步：创建 `theme.tokens`

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

### 第二步：创建 `theme.toml`

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

### 第三步：创建 `build.rs`

```rust
use spectrum_codegen::ThemeCodegen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ThemeCodegen::from_contract("theme.tokens", "theme.toml").generate()?;
    Ok(())
}
```

### 第四步：在源码中引入

```rust
include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));

fn main() {
    let theme = AppTheme::try_load().unwrap();
    let hover = theme.button_state.hover;
}
```

`from_contract` 会在构建期校验外部契约和 TOML 语法。生成的 `try_load` 使用
`TomlThemeSource`，因此消费 crate 需要启用 `spectrum-theme` 的 `toml` feature。

### 多个主题文件

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

### ThemeCodegen 完整配置

```rust
ThemeCodegen::from_contract("src/theme.tokens", "src/theme.toml")
    .output_file("theme_tokens.rs")
    .facade_path("::spectrum_theme")
    .generate()?;
```

---

## 运行时 API 参考

所有生成的结构体都拥有以下方法：

| 方法 | 签名 | 何时可用 |
|---|---|---|
| `try_from_source` | `fn try_from_source<S: TokenSource>(source: &S) -> Result<Self, S::Error>` | 始终 |
| `reload` | `fn reload<S: TokenSource>(&mut self, source: &S) -> Result<(), S::Error>` | 始终 |
| `try_load` | `fn try_load() -> Result<Self, ThemeBuildError>` | build.rs 路径 |

`try_from_source` 和 `reload` 还要求契约中的每个 token 值类型都为传入的 source 实现
`ThemeValue<S>`。

```rust
// ─── 手写 DSL 路径（无嵌入数据） ───
let mut theme = AppTheme::try_from_source(&source).unwrap();
theme.reload(&new_source).unwrap();

// ─── build.rs 路径（有嵌入数据） ───
let mut theme = AppTheme::try_load().unwrap();                             // 嵌入默认主题
theme.reload(&other_source).unwrap();                                       // 整主题替换
```

---

## TOML 配置参考

### 元数据 `[meta]`

| 字段 | 类型 | 必填 |
|---|---|---|
| `name` | string | ✅ |
| `mode` | `"dark"` 或 `"light"` | 否，默认 `"dark"` |
| `author` | string | 否 |
| `version` | string | 否 |
| `description` | string | 否 |

### 种子颜色 `seed`

可选。提供后，Material role 引用（如 `{material.primary}`）才能解析。

```toml
seed = "#6750a4"       # RGB
seed = "#6750a480"     # RGBA
```

运行时代码可以覆盖 TOML 中的 seed，而不修改 token 配置：

```rust
let source = TomlThemeSource::parse(include_str!("theme.toml"))?
    .with_seed(Color::new(0, 0, 255));
let theme = AppTheme::try_from_source(&source)?;
```

从外部 contract 文件生成的主题也会提供 seed 专用加载方法：

```rust
let mut theme = AppTheme::try_load_with_seed(Color::new(0, 0, 255))?;
theme.try_set_seed(Color::new(255, 0, 0))?;
```

### Contract Token 表

token path 来自生成契约。嵌套字段映射为 TOML 表，最后一级字段名是标量 key：

```toml
[surface]
bg = "#1e1e2e"

[accent]
primary = "{material.primary}"

[border]
focus = "{accent.primary}"
```

无状态组件实例和状态集合也使用同样规则：

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

缺失的状态字段会沿生成契约里的 `extends` 链继承。因此上面的
`nav_button.hover.fg` 和 `nav_button.hover.radius` 会读取 `nav_button.normal` 中的值。

### Material 颜色角色

| 角色 | 用途 |
|---|---|
| `material.primary` | 主品牌色 |
| `material.on_primary` | 主品牌色上的内容色 |
| `material.primary_container` | 主品牌色的容器色 |
| `material.on_primary_container` | 容器上的内容色 |
| `material.secondary` | 次要色 |
| `material.on_secondary` | 次要色上的内容色 |
| `material.secondary_container` | 次要色的容器色 |
| `material.on_secondary_container` | 次要容器上的内容色 |
| `material.tertiary` | 第三色 |
| `material.on_tertiary` | 第三色上的内容色 |
| `material.tertiary_container` | 第三色的容器色 |
| `material.on_tertiary_container` | 第三容器上的内容色 |
| `material.background` | 背景色 |
| `material.on_background` | 背景上的内容色 |
| `material.surface` | 表面色 |
| `material.on_surface` | 表面上的内容色 |
| `material.surface_dim` | 最暗表面色 |
| `material.surface_bright` | 最亮表面色 |
| `material.surface_container_lowest` | 最低容器色 |
| `material.surface_container_low` | 低容器色 |
| `material.surface_container` | 默认容器色 |
| `material.surface_container_high` | 高容器色 |
| `material.surface_container_highest` | 最高容器色 |
| `material.surface_variant` | 表面变体色 |
| `material.on_surface_variant` | 表面变体上的内容色 |
| `material.inverse_surface` | 反转表面色 |
| `material.inverse_on_surface` | 反转表面上的内容色 |
| `material.outline` | 轮廓色 |
| `material.outline_variant` | 轮廓变体色 |
| `material.shadow` | 阴影色 |
| `material.scrim` | 遮罩色 |
| `material.surface_tint` | 表面色调 |
| `material.inverse_primary` | 反转主色 |
| `material.primary_fixed` | 固定主色 |
| `material.primary_fixed_dim` | 暗固定主色 |
| `material.on_primary_fixed` | 固定主色上的内容色 |
| `material.on_primary_fixed_variant` | 固定主色变体上的内容色 |
| `material.secondary_fixed` | 固定次要色 |
| `material.secondary_fixed_dim` | 暗固定次要色 |
| `material.on_secondary_fixed` | 固定次要色上的内容色 |
| `material.on_secondary_fixed_variant` | 固定次要变体上的内容色 |
| `material.tertiary_fixed` | 固定第三色 |
| `material.tertiary_fixed_dim` | 暗固定第三色 |
| `material.on_tertiary_fixed` | 固定第三色上的内容色 |
| `material.on_tertiary_fixed_variant` | 固定第三变体上的内容色 |
| `material.error` | 错误色 |
| `material.on_error` | 错误色上的内容色 |
| `material.error_container` | 错误容器色 |
| `material.on_error_container` | 错误容器上的内容色 |

### 标量值

内置值会按照契约字段类型从标量字符串解析。自定义值可以实现
`ThemeValue<TomlThemeSource>`，并解析 `source.token_text(path)` 返回的原始字符串。

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

### 阴影令牌

```toml
[shadow.card]
color = "#00000033"
offset_x = "0px"
offset_y = "4px"
blur = "8px"
spread = "0px"
```

---

## 自定义类型扩展

内置类型（`Color`、`Length` 等）并非封闭的。可以添加你自己的令牌类型：

```rust
use spectrum_theme::source::{ThemeValue, TokenSource};

// ① 定义你的类型
#[derive(Debug, Clone, Copy)]
pub struct Padding(pub u16);

// ② 在 DSL 中使用
define_theme_tokens! {
    pub struct CustomTheme {
        spacing {
            pad: Padding,  // ← 自定义类型
        }
    }
}

// ③ 教某个 Source 如何提供它
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

> **模式**：每个值类型通过 `impl ThemeValue<MySource> for MyValue` 接入具体 source。
> 生成代码只调用 `source.token::<MyValue>("path")`。

---

## Feature 标志

```toml
[dependencies]
spectrum-theme = { version = "0.2.0", features = ["macros", "toml", "seed"] }
```

| Feature | 作用 |
|---|---|
| `macros` | 启用 `define_theme_tokens!` 宏 |
| `seed` | 启用 Material 颜色派生（`material.primary` 等） |
| `serde` | 为核心类型启用 serde 序列化 |
| `toml` | 启用 contract-aware TOML source 加载 |
| `ratatui` | 启用 Ratatui adapter 重导出 |
| `iced` | 启用 iced adapter 重导出 |
