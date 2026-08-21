# CLI

AIX CLI 用于创建、检查、优化和预览 `.aix` 产物。npm 与原生 Rust 两个版本都提供同一个 `aix` 命令，并共享 Rust 打包引擎，因此可以按开发环境任选其一。

## 安装

使用 npm 安装，无需 Rust 工具链：

```bash
npm install -g @yodaos-pkg/aix-cli
```

或安装原生 Rust 二进制文件：

```bash
cargo install aiui-aix-cli
```

安装后确认命令可用：

```bash
aix --help
```

## 快速开始

以下命令会将 `my-agent` 目录打包到 `dist/`，列出包内条目，然后启动本地预览：

```bash
aix pack ./my-agent -o ./dist/my-agent.aix
aix list ./dist/my-agent.aix
aix preview ./dist/my-agent.aix --launch
```

## 打包目录

基本用法：

```bash
aix pack ./my-agent
```

默认输出为当前目录下的 `bundle.aix`。使用 `--output` 或 `-o` 指定路径：

```bash
aix pack ./my-agent -o ./dist/my-agent.aix
```

写入归档前，打包器会：

- 应用 `.aixignore` 和标准忽略规则
- 校验 JSON 文件
- 要求存在 `app.json`
- 检查 `app.json.pages` 是否为非空页面路径数组，不符合时给出警告
- 按需将支持的文本文件转换为 UTF-8
- 生成唯一的 `VERSION` 构建 ID
- 生成包含包元数据和文件摘要的 `META-INF/aix/manifest.json`

### 声明引擎范围

使用 `--engine` 声明可以运行该包的 AIX 引擎版本：

```bash
aix pack ./my-agent --engine '^0.14.0'
```

常见写法：

```text
*           任意版本
0.14.0      仅 0.14.0
>=0.14.0    0.14.0 或更高版本
^0.14.0     与 0.14.0 兼容的版本
```

打包器会校验版本范围，并将其写入 `META-INF/aix/manifest.json`。若未传入 `--engine`，则依次使用 `app.json.engine` 和默认值 `*`。包生成后，运行时只从清单读取兼容范围。

### 打包时优化

使用 `--optimize` 或 `-O` 优化 JSON、PNG 和 JPEG：

```bash
aix pack ./my-agent -O
aix pack ./my-agent -O --opt-level 3
```

优化等级为 1 到 3，默认为 2。优化只影响输出包，不会修改源目录。

### 参数一览

```text
Usage: aix pack [OPTIONS] <INPUT_DIR>

Arguments:
  <INPUT_DIR>  要打包的输入目录

Options:
  -o, --output <OUTPUT_FILE>  输出文件 [default: bundle.aix]
  -O, --optimize              启用优化
      --opt-level <LEVEL>     优化等级，1-3 [default: 2]
      --engine <RANGE>        支持的引擎版本范围
  -h, --help                  显示帮助
```

## 检查包内容

列出归档条目及其原始大小和压缩后大小：

```bash
aix list ./bundle.aix
```

`ls` 是等价的简写：

```bash
aix ls ./bundle.aix
```

该命令只显示归档条目。引擎范围等包级元数据位于 `META-INF/aix/manifest.json`，不应从 `app.json` 读取。

需要查看文件内容、页面和工具定义时，可以使用[在线工具](/play.zh-CN)。

## 优化已有包

为现有产物创建优化副本：

```bash
aix optimize ./bundle.aix -o ./bundle.optimized.aix
aix optimize ./bundle.aix -o ./bundle.optimized.aix --level 3
```

优化器会保留原有引擎范围。由于包内容发生变化，它会移除旧签名并生成新的未签名清单；需要签名验证时，应在优化后重新签名。

## 在浏览器中预览

`preview` 可以读取 `.aix` 产物，也可以直接读取源目录：

```bash
aix preview ./bundle.aix
aix preview ./my-agent
```

默认行为如下：

- 启动本地 HTTP 服务并输出预览 URL
- 不自动打开浏览器
- 将当前内容快照嵌入单个 HTML 文档
- 运行时从网络加载 Ink SDK
- 使用固定的 `480x352` 预览视口

添加 `--launch` 可自动打开默认浏览器：

```bash
aix preview ./bundle.aix --launch
```

预览已打包产物时，元数据来自 `META-INF/aix/manifest.json`；预览目录时则直接读取源树。

### 导出静态 HTML

使用 `--html-out` 将预览写入文件，而不启动服务器：

```bash
aix preview ./bundle.aix --html-out ./artifacts/preview.html
```

此模式会自动创建所需的父目录，且不能与 `--launch` 同时使用。

生成的页面通过 import map 从 `jspm.io` 加载 Ink 浏览器运行时。运行时版本优先使用 `aix runtime select` 保存的选择，否则回退到当前 `AIX_NPM_REGISTRY` 的 `latest` 版本。预览侧栏会显示实际使用的 Ink 版本。

### 开发模式

使用 `--dev` 监听包或目录变化：

```bash
aix preview ./bundle.aix --dev
aix preview ./my-agent --dev
aix preview ./my-agent --dev --launch
```

开发模式会：

- 始终启动本地预览服务器
- 从服务器获取预览状态
- 通过 WebSocket 接收变更通知
- 在 `.aix` 文件或源目录变化时实时重载
- 重建 `InkView`，而不刷新整个文档

`--dev` 不能与 `--html-out` 同时使用。

## 管理预览运行时

`runtime` 命令组仅用于 npm CLI 的 Ink 预览运行时。

### 选择注册表

使用 `AIX_NPM_REGISTRY` 选择包元数据来源：

```bash
AIX_NPM_REGISTRY=npm aix runtime versions
AIX_NPM_REGISTRY=npmmirror aix runtime versions
```

支持 `npm`（默认）和 `npmmirror`。

### 查看可用版本

```bash
aix runtime versions
```

输出包括运行时来源、包名、当前版本、已保存的选择（如有），以及 `@yodaos-pkg/ink` 已发布的稳定版本。

### 输出当前版本

```bash
aix runtime current
```

该命令只输出解析后的版本字符串，适合在 Shell 脚本中使用。它优先读取 `aix runtime select` 保存的版本。

### 交互式选择版本

```bash
aix runtime select
```

选择结果保存在 `~/.aix/runtime.json`。列表只显示稳定版本，并用 `(selected)` 标记本地选择；当注册表默认版本不同时，用 `(current default)` 标记默认版本。

## 在源码工作区运行

开发仓库代码时，可以不做全局安装：

```bash
# npm 入口
cd packages/cli
npm install
npm run build
node dist/cli.js pack ./my-agent -o bundle.aix

# 原生 Rust 入口
cargo run -p aiui-aix-cli -- pack ./my-agent -o bundle.aix
```

在任意子命令后传入 `--help` 可查看当前参数：

```bash
node dist/cli.js pack --help
node dist/cli.js runtime --help
```

## 继续阅读

- [格式规范](/spec.zh-CN)：包结构与兼容性模型
- [代码组件](/packages.zh-CN)：CLI、读取库和 Web/WASM 的关系
- [在线工具](/play.zh-CN)：在浏览器中构建或检查产物
