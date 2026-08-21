# 代码组件

AIX 仓库以 Rust 工作区为核心，并向命令行、npm 和浏览器提供一致的格式能力。各组件共享同一套包模型，但承担不同职责。

## 职责总览

| 路径 | 主要职责 | 适用场景 |
| --- | --- | --- |
| `crates/aix` | 读取与分析 | 在 Rust 中检查包、页面和工具定义 |
| `crates/aix-pack` | 构建与优化 | 从内存文件生成规范化的 AIX 产物 |
| `crates/aix-cli` | 原生命令行 | 使用 Rust 二进制打包、检查和预览 |
| `packages/cli` | npm 命令行 | 在 Node.js 环境使用相同的 `aix` 工作流 |
| `crates/aix-web` | Web/WASM 接口 | 在浏览器和 TypeScript 中读取或构建包 |

依赖方向可以概括为：读取模型由 `crates/aix` 提供，写入和优化能力集中在 `crates/aix-pack`，其余组件把这些能力暴露到不同运行环境。

## `crates/aix`

核心读取与分析库负责：

- 枚举和读取包内条目
- 获取版本、标题等元数据
- 解析应用路由和页面信息
- 从页面 schema 推导工具定义
- 检查运行时是否满足包的引擎范围

需要理解 AIX 格式本身或在 Rust 程序中消费 `.aix` 文件时，应从此 crate 开始。

禁用默认 feature 后，它支持 `no_std + alloc`，可用于嵌入式和 RTOS 环境。

## `crates/aix-pack`

共享的内存构建层同时服务于原生环境和 Web/WASM。它负责：

- 从输入文件构建 AIX 归档
- 规范化文本编码
- 压缩 JSON
- 使用纯 Rust 编解码器优化 PNG 和 JPEG
- 生成清单及内容摘要

将写入逻辑集中在这里，可以让原生 CLI、npm CLI 和浏览器工具生成一致的产物。

## `crates/aix-cli`

原生 Rust CLI 的包名为 `aiui-aix-cli`，安装后命令名为 `aix`：

```bash
cargo install aiui-aix-cli
```

它将核心能力组合为面向开发者的命令，包括目录打包、包内容检查、资源优化和本地预览。

## `packages/cli`

npm 包 `@yodaos-pkg/aix-cli` 提供与原生版本相同的 `aix` 命令：

```bash
npm install -g @yodaos-pkg/aix-cli
```

它是同一 Rust 引擎之上的轻量 TypeScript 外壳，底层使用 Node.js WASM 包。没有 Rust 工具链，或需要集成 Node.js 工作流时，优先使用此入口。

两种 CLI 的命令语义一致，具体用法见 [CLI 文档](/cli.zh-CN)。

## `crates/aix-web`

Web 包通过 WASM 和 TypeScript API 暴露 AIX 模型，典型能力包括：

- 从 `Uint8Array` 或 `File` 打开包
- 列出条目并读取原始文件
- 获取版本、标题和页面
- 生成工具定义
- 检查引擎兼容性
- 在浏览器中构建 AIX 产物

本站的[在线工具](/play.zh-CN)就是该组件的直接使用示例。

## 开发与验证

修改核心格式、CLI 或 Web 接口后，典型验证命令为：

```bash
cargo test -p aiui-aix -p aiui-aix-cli
cargo check -p aiui-aix-web --target wasm32-unknown-unknown
```

三层内容之间的关系很明确：文档解释格式，代码组件实现格式，在线工具使用真实 `.aix` 产物验证格式。

继续阅读[格式规范](/spec.zh-CN)，或打开[在线工具](/play.zh-CN)直接检查一个包。
