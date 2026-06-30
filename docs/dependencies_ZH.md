# 依赖评估

初始依赖集特意保持精简并集中在根工作区清单中。

| 依赖 | 用途 | 决策 |
| --- | --- | --- |
| `serde` | 可选的 core 序列化 | 引入 |
| `toml` | contract-aware TOML source 加载 | 可选 feature |
| `thiserror` | 稳定的错误分类和来源追溯 | 引入 |
| `proc-macro2` | 过程宏 token 模型 | 引入到 macro crate |
| `quote` | 过程宏输出 | 引入到 macro crate |
| `syn` | 过程宏输入解析 | 引入到 macro crate |
| `palette` | 色彩空间转换原语 | 可选 feature |
| `material-colors` | 从种子颜色生成 Material 颜色角色 | 可选 `seed` feature |
| `ratatui` | 终端 UI 适配器目标 | 适配器 crate 依赖 |
| `iced_core` | Iced 形式化转换目标 | 适配器 crate 依赖 |
| `iced` | Iced 可视化示例运行时 | 可选适配器 feature |

## 种子颜色算法

`palette` crate 仍然可用于通用色彩空间转换。种子颜色到 Material 角色的路径目前使用 `material-colors`，通过可选的 `seed` feature 引入。任何替换方案需满足：

1. 针对已发布的 Material 参考值的算法兼容性测试。
2. 兼容 Rust 1.88 的发行版。
3. 可接受的维护状态、许可证和依赖规模。
4. 适合快照和契约测试的确定性输出。

## 延迟的依赖

egui、syntect、CSS 辅助依赖以及 Design Tokens 导出依赖均延迟到其适配器或导出契约实现之后。
