# 工作区架构

## 依赖方向

```text
spectrum-core
    ^
    |
spectrum-palette
    ^
    |
spectrum-codegen
    |
    v
build.rs 输出 ---> spectrum-theme

spectrum-codegen <- spectrum-macros -------------------> spectrum-theme
spectrum-core <- spectrum-ratatui <- spectrum-theme
spectrum-core <- spectrum-iced ----^
```

依赖图必须保持无环。`spectrum-core` 是最底层的 crate，不得依赖配置格式、宏解析或渲染框架。

## Crate 边界

### `spectrum-core`

拥有平台无关的值类型。可以暴露可选的序列化支持，但不能解析主题文件。

### `spectrum-palette`

拥有"种子颜色 → 色调调色板"的边界。`seed` feature 使用 `material-colors` 生成 Material 风格的角色，同时将算法依赖隔离在 `spectrum-core` 之外。

### `spectrum-codegen`

拥有类型化令牌契约的编译期 Rust 源码生成能力。它在构建脚本中解析并校验外部 contract 文件和 contract-aware TOML 取值文件，生成普通 Rust 源文件供消费 crate 从 `OUT_DIR` include，使 IDE 诊断和补全可以感知生成的令牌字段。

内联契约还可以定义可复用组件结构和状态集合。例如 `ButtonTokens` 只生成一次，而
`button` 状态集合会生成 `normal`、`hover`、`focus` 等字段，并且这些字段都使用同一个
组件类型。状态集合让 UI 状态关系进入生成的 Rust 契约，而不是把每个状态路径都变成互不兼容的嵌套结构体类型。声明的 `extends` 关系也会驱动生成代码在状态字段缺失时按父状态回退读取。

codegen 负责把 contract path 映射到可复用 Rust 结构体，并在构建 typed theme 时应用状态回退；平台 adapter 不需要知道 button、focus 状态或动画语义。

### `spectrum-macros`

拥有内联类型化令牌契约的过程宏。文件驱动的代码生成属于 `spectrum-codegen`，这样宏展开不会对 rust-analyzer 隐藏文件内容。

### `spectrum-theme`

提供面向用户的公共门面，协调可选功能。它自身应包含尽可能少的行为。

### `spectrum-ratatui`

拥有 Ratatui 的核心值转换。它暴露适配器 trait，将终端特定的选择隔离在 `spectrum-core` 之外。

### `spectrum-iced`

拥有 Iced 的核心值转换。该 crate 依赖 `iced_core` 进行形式化转换，并将完整的可视化示例置于 `runtime` feature 之后。

## 延迟的适配器和导出

CSS、Design Tokens JSON、egui、syntect 以及其他平台输出不属于 `0.2.0`。只有在它们的转换契约具体化之后，才应作为独立的 crate 或 feature 添加。
