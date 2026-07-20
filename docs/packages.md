# Packages

The AIX repository is organized as a Rust workspace with three format-facing surfaces.

## `crates/aix`

This crate is the core reading and analysis layer.

It is responsible for the package-facing concepts that define the AIX model:

- reading package entries
- surfacing metadata like version and title
- resolving page information
- deriving tool definitions from page schema

If you want to understand the format itself, start here.

## `crates/aix-cli`

The CLI is the packaging and inspection surface.

It turns the format into developer workflows such as:

- packaging directories into `.aix`
- validating and normalizing packaged assets
- listing package contents

This is the operational bridge between source files and the final format artifact.

## `crates/aix-web`

This package exposes AIX in browser-facing environments through WASM.

It makes the format readable inside interactive interfaces, including the official package lab published with this site.

## Development Flow

Typical validation flow:

```bash
cargo test -p aix -p aix-cli
cargo check -p aix-web --target wasm32-unknown-unknown
```

## Relationship To The Website

The docs site explains the model.

The packages implement the model.

The Package Lab demonstrates the model against real `.aix` artifacts.

## Explore Further

- Read the [Specification](/spec)
- Open the [Package Lab](/play)
