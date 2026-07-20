# AIX

`aix` is a toolchain built around the **`.aix` (AI eXecutable)** package format for Ink Mini Program / Agent scenarios. It provides:

- A Rust core library for reading, parsing, and analyzing `.aix` packages
- A CLI for packing directories into `.aix` files and inspecting package contents
- Web/WASM bindings for reading `.aix` packages in browsers and TypeScript environments

An `.aix` file is essentially a zip archive that contains agent definitions, page assets, and runtime metadata. The current implementation in this repository lives under `crates/`.

## Repository Layout

```text
crates/
├── aix/         # Rust core library: AixReader, page analysis, tool generation
├── aix-cli/     # Command-line tool: aix pack / aix list
└── aix-web/     # WebAssembly + TypeScript bindings for browser-facing AIX tooling
```

## Modules

### `crates/aix`

The core Rust library parses `.aix` packages and exposes a unified reading API. Its main capabilities include:

- Listing files inside a package
- Reading a specific file
- Reading the `VERSION` file
- Extracting the app title from `app.json`
- Parsing page definitions and extracting page `schema`
- Analyzing page size constraints from page structure and styles
- Generating OpenAI-compatible tool definitions from pages

The library currently supports two page formats:

1. Traditional multi-file format
   - `page.json`
   - `page.js`
   - `page.wxml`
   - `page.wxss` / `page.wcss`
2. Single-file component format
   - `page.ink`

### `crates/aix-cli`

The command-line tool exposes the `aix` binary and is mainly used to pack and inspect `.aix` files.

Current features:

- `aix pack <INPUT_DIR>`
  - Packs a directory into an `.aix` file
  - Automatically generates a root-level `VERSION` file
  - Validates all `.json` files before packaging
  - Converts non-UTF-8 `.json`, `.js`, and `.ink` files to UTF-8 before writing them into the archive
  - Supports PNG / JPEG optimization and JSON minification
  - Respects `.aixignore`
- `aix list <AIX_FILE>`
  - Lists package files and their original / compressed sizes
  - Supports the alias `aix ls`

### `crates/aix-web`

This module builds on top of the `aix` core library and provides WebAssembly bindings plus a TypeScript API for reading `.aix` files in web environments.

Main capabilities:

- `AIX.From(data)` initializes from a `Uint8Array` or `File`
- `list()` returns the file list
- `readFile(name)` reads raw file content
- `getVersion()` returns the package version
- `getTitle()` returns the app title
- `getPages()` returns parsed page information
- `getTools()` returns generated tool definitions

The official browser inspection surface is the docs-integrated Package Lab at `/play`.

## AIX Package Format

An `.aix` package is a zip-based application bundle for Ink Mini Program / Agent scenarios. It is used to package:

- Agent metadata and capability descriptions
- App-level configuration and runtime entry files
- Page definitions and UI resources
- Optional page schemas that can be turned into tool definitions

In practice, the format supports both traditional multi-file pages and `.ink` single-file components, allowing the same package to be consumed by Rust, CLI, and Web/WASM tooling in this repository.

## AIX Package Structure

A typical `.aix` package usually contains:

```text
.
├── AGENTS.md
├── app.json
├── app.js
└── pages/
```

Where:

- `AGENTS.md` describes the agent identity and capabilities
- `app.json` contains app configuration, routing, and window metadata
- `app.js` is the app logic entry
- `pages/` contains page definitions

## Quick Start

### 1. Read `.aix` in Rust

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

### 2. Pack a Directory with the CLI

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- pack ./my-agent -o bundle.aix
```

Enable optimization:

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- pack ./my-agent -o bundle.aix -O --opt-level 3
```

Inspect package contents:

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- list ./bundle.aix
```

### 3. Read `.aix` on the Web

```ts
import { AIX } from '@yodaos-pkg/aix';

async function inspect(file: File) {
  const aix = await AIX.From(file);
  console.log(aix.getTitle());
  console.log(aix.getPages());
  console.log(aix.getTools());
}
```

## Local Development

### Rust Core Library

Run tests for `aix`:

```bash
cargo test --manifest-path crates/aix/Cargo.toml
```

### CLI

Run tests for `aix-cli`:

```bash
cargo test --manifest-path crates/aix-cli/Cargo.toml
```

Run the CLI locally:

```bash
cargo run --manifest-path crates/aix-cli/Cargo.toml -- --help
```

### Web / WASM

Install dependencies:

```bash
cd crates/aix-web
npm install
```

Build the WASM and TypeScript outputs:

```bash
npm run build
```

Install docs dependencies:

```bash
cd ../../docs
npm install
```

Start the docs site:

```bash
npm run dev
```

## Current Status

- The main code currently lives under `crates/`
- The repository root already has a `Cargo.toml`, but these crates are not yet registered as workspace members
- If you want to run `cargo test`, `cargo run -p ...`, or share workspace dependencies from the root, the workspace configuration can be completed later

## License

MIT
