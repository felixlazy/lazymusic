# Lazymusic

一个用 Rust 编写的终端音乐播放器。

## 项目结构

本项目采用 Cargo workspace 进行管理，主要由以下几个部分组成：

- `lazy-app`: 主应用程序，负责启动和管理整个应用。
- `lazy-core`: 核心逻辑、数据结构和 traits。
- `lazy-tui`: 终端用户界面 (TUI) 相关组件，基于 `ratatui` 构建。
- `lazy-macro`: 项目中使用的过程宏。

## 如何构建和运行

1. 确保你已经安装了 [Rust 环境](https://www.rust-lang.org/tools/install)。
2. 克隆本项目。
3. 在项目根目录下执行以下命令：

   ```bash
   # 构建项目
   cargo build --release

   # 运行项目
   cargo run --release
   ```

## 主要依赖

- [ratatui](https://github.com/ratatui-org/ratatui): 用于构建终端用户界面。
- [tokio](https://github.com/tokio-rs/tokio): 提供异步运行时。
- [crossterm](https://github.com/crossterm-rs/crossterm): 用于处理终端事件和样式。

## 许可证

本项目采用 [MIT](LICENSE) 许可证。
