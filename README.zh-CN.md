# AIX

`aix` 是一组围绕 **`.aix` (AI eXecutable)** 包格式的工具链，面向 Ink Mini Program / Agent 场景，提供：

- Rust 核心库：读取、解析和分析 `.aix` 包
- CLI 工具：将目录打包为 `.aix`，或查看包内容
- Web/WASM 绑定：在浏览器和 TypeScript 环境中读取 `.aix`

`.aix` 本质上是一个 zip 包，承载 Agent 定义、页面资源和运行时元信息。当前仓库里的相关实现都位于 `crates/` 下。

## 仓库结构

```text
crates/
├── aix/         # Rust 核心库：AixReader、页面分析、工具定义生成
├── aix-cli/     # 命令行工具：aix pack / aix list
└── aix-web/     # WebAssembly + TypeScript 封装，以及本地 playground
```

## 模块说明

### `crates/aix`

核心 Rust 库，负责解析 `.aix` 包并暴露统一读取接口，主要能力包括：

- 列出包内文件
- 读取指定文件
- 获取 `VERSION`
- 从 `app.json` 读取应用标题
- 解析页面定义并提取 `schema`
- 基于页面结构和样式分析页面尺寸约束
- 为页面生成 OpenAI-compatible tool 定义

当前同时支持两种页面格式：

1. 传统多文件格式
   - `page.json`
   - `page.js`
   - `page.wxml`
   - `page.wxss` / `page.wcss`
2. 单文件组件格式
   - `page.ink`

### `crates/aix-cli`

命令行工具，二进制名称为 `aix`，主要用于打包和检查 `.aix` 文件。

当前支持：

- `aix pack <INPUT_DIR>`
  - 将目录打成 `.aix`
  - 自动生成根 `VERSION` 文件
  - 校验 `.json` 文件是否合法
  - 自动将 `.json` / `.js` / `.ink` 的非 UTF-8 内容转换为 UTF-8 后再写入包内
  - 支持 PNG / JPEG 优化和 JSON 压缩
  - 支持 `.aixignore`
- `aix list <AIX_FILE>`
  - 列出包内文件及压缩前后大小
  - 支持别名 `aix ls`

### `crates/aix-web`

基于 `aix` 核心库提供 WebAssembly 封装，并在 `ts/` 中暴露 TypeScript API，适合在浏览器中读取 `.aix` 文件。

主要能力包括：

- `AIX.From(data)` 从 `Uint8Array` 或 `File` 初始化实例
- `list()` 获取包中文件列表
- `readFile(name)` 读取原始文件内容
- `getVersion()` 获取版本号
- `getTitle()` 获取应用标题
- `getPages()` 获取页面信息
- `getTools()` 获取页面导出的 tool 定义

仓库里还包含一个本地 playground，便于在浏览器里调试 `.aix` 包内容与页面解析结果。

## AIX 包格式

`.aix` 包是一种面向 Ink Mini Program / Agent 场景的 zip 形式应用包，用来承载：

- Agent 元数据与能力描述
- 应用级配置与运行时入口文件
- 页面定义与 UI 资源
- 可进一步转成 tool 定义的页面 schema

在实际使用中，这一格式同时支持传统多文件页面和 `.ink` 单文件组件，因此可以被本仓库中的 Rust、CLI 与 Web/WASM 工具链统一消费。

## AIX 包结构

一个典型的 `.aix` 包通常包含以下文件：

```text
.
├── AGENTS.md
├── app.json
├── app.js
└── pages/
```

其中：

- `AGENTS.md` 描述 Agent 身份与能力
- `app.json` 描述应用配置、路由和窗口信息
- `app.js` 是应用逻辑入口
- `pages/` 存放页面定义

## 快速开始

### 1. 在 Rust 中读取 `.aix`

```rust
use aix::AixReader;

fn main() -> anyhow::Result<()> {
    let data = std::fs::read("bundle.aix")?;
    let reader = AixReader::new(data)?;

    for entry in reader.list() {
        println!("{} ({})", entry.name, entry.size);
    }

    println!("title = {:?}", reader.get_title());
    println!("version = {:?}", reader.get_version());
    println!("pages = {:?}", reader.get_pages());
    println!("tools = {:?}", reader.get_tools());

    Ok(())
}
```

### 2. 用 CLI 打包目录

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- pack ./my-agent -o bundle.aix
```

开启优化：

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- pack ./my-agent -o bundle.aix -O --opt-level 3
```

查看包内容：

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- list ./bundle.aix
```

### 3. 在 Web 中读取 `.aix`

```ts
import { AIX } from '@yodaos-pkg/aix';

async function inspect(file: File) {
  const aix = await AIX.From(file);
  console.log(aix.getTitle());
  console.log(aix.getPages());
  console.log(aix.getTools());
}
```

## 本地开发

### Rust 核心库

运行 `aix` 测试：

```bash
cargo test --manifest-path crates/aix/Cargo.toml
```

### CLI

运行 `aix-cli` 测试：

```bash
cargo test --manifest-path crates/aix-cli/Cargo.toml
```

本地执行 CLI：

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- --help
```

### Web / WASM

安装依赖：

```bash
cd crates/aix-web
npm install
```

构建 WASM 与 TS 输出：

```bash
npm run build
```

启动 playground：

```bash
npm run dev
```

## 当前状态

- 当前仓库代码主体位于 `crates/`
- 根目录已经存在 `Cargo.toml`，但还没有把这些 crate 注册为 workspace members
- 如果后续希望统一通过根目录执行 `cargo test`、`cargo run -p ...` 或共享 workspace 依赖，可以再补齐根 workspace 配置

## License

MIT
