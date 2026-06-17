# Void Spectrum

[English](README.md)

Void Spectrum 是一个用 Rust 编写的类型化主题令牌引擎工作区。

## 工作区

| Crate | 职责 |
| --- | --- |
| `spectrum-core` | 平台无关的值类型和已解析的主题类型 |
| `spectrum-schema` | 可序列化的主题规范与配置格式 |
| `spectrum-palette` | 种子颜色与色调调色板生成的边界 |
| `spectrum-resolver` | 合并、引用解析、校验与错误报告 |
| `spectrum-codegen` | 编译期类型化令牌代码生成 |
| `spectrum-macros` | 内联类型化令牌契约生成 |
| `spectrum-ratatui` | Ratatui 适配器 trait 与终端可视化示例 |
| `spectrum-iced` | Iced 适配器 trait 与 GUI 可视化示例 |
| `spectrum-theme` | 公共门面与特性协调 |

## Facade 特性

`spectrum-theme` 默认启用基于种子颜色的 Material 颜色生成。可选 feature 暴露聚焦的 API：

| Feature | 启用 |
| --- | --- |
| `macros` | `define_theme_tokens!` |
| `toml` | TOML schema 加载支持 |
| `json` | JSON schema 加载支持 |
| `ratatui` | `spectrum_theme::ratatui` 适配器重导出 |
| `iced` | `spectrum_theme::iced` 适配器重导出 |

## 类型化主题生成

`define_theme_tokens!` 定义类型化契约，可以从运行时加载的主题中填充数据。对于静态 TOML 主题，在 `build.rs` 中生成 Rust 代码，这样 rust-analyzer 可以通过 Cargo 的构建脚本输出检查生成的结构体和字段：

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    spectrum_codegen::ThemeCodegen::new("themes/app.toml", "AppTheme")
        .generate()?;
    Ok(())
}
```

然后在 Rust 代码中 include 生成的源文件：

```rust
include!(concat!(env!("OUT_DIR"), "/theme_tokens.rs"));
```

## 开发

```bash
cargo check --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --no-deps
```

参见 [架构](docs/architecture_ZH.md) 了解依赖方向，
[依赖](docs/dependencies_ZH.md) 了解初始依赖评估，以及 [使用指南](docs/usage_ZH.md) 了解基本用法。
