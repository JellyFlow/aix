---
title: AIX
layout: doc
aside: false
---

# AIX

AIX 是面向 AI 应用的可分发文件格式。它把应用入口、页面、资源、数据约定和运行时要求打包为一个 `.aix` 文件，同时保留足够的结构信息，供命令行、Rust 程序和浏览器工具读取。

## 从这里开始

只想创建和查看一个包，可以先安装 CLI：

```bash
npm install -g @yodaos-pkg/aix-cli
aix pack ./my-agent -o ./dist/my-agent.aix
aix list ./dist/my-agent.aix
```

也可以安装原生 Rust 版本。两种安装方式提供相同的 `aix` 命令：

```bash
cargo install aiui-aix-cli
```

## AIX 解决什么问题

普通归档文件只能保存字节和目录结构。AIX 在此基础上保存应用语义，使同一个包能够被不同工具一致地理解：

- `app.json` 描述应用信息和页面路由
- `pages/` 保存页面实现、资源和数据 schema
- `META-INF/aix/manifest.json` 记录包身份、引擎范围、文件大小和摘要
- 页面 schema 可以进一步转换为结构化工具定义

因此，`.aix` 既是交付物，也是可以检查、验证和继续处理的应用描述。

## 文档路线

- [格式规范](/spec.zh-CN)：了解包结构、读取顺序和兼容性模型
- [CLI](/cli.zh-CN)：打包、检查、优化和预览 `.aix` 文件
- [代码组件](/packages.zh-CN)：了解 Rust、WASM 和 npm 入口的职责
- [在线工具](/play.zh-CN)：在浏览器中构建或检查包

## 典型工作流

1. 在目录中编写 `app.json`、`app.js` 和页面文件。
2. 使用 `aix pack` 校验内容并生成 `.aix` 产物。
3. 使用 `aix list`、浏览器工具或读取库检查产物。
4. 在发布前按需优化并签名。
5. 运行时根据包清单判断引擎兼容性，再加载页面和工具定义。

下一步建议从[格式规范](/spec.zh-CN)开始；如果已经有源目录，可以直接查看 [CLI 快速开始](/cli.zh-CN#快速开始)。
