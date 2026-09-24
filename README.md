# zed-navigator-lsp

A [Zed](https://zed.dev) extension shim that attaches
[`navigator-lsp`](https://github.com/neon-law-source-code/navigator/blob/main/docs/lsp/README.md) to Markdown
buffers.

This repository carries no Navigator source. At runtime the extension resolves the latest `navigator-lsp-<tag>-<platform>`
archive from [`neon-law-source-code/navigator`'s GitHub Releases](https://github.com/neon-law-source-code/navigator/releases),
downloads it, and launches the `navigator-lsp` binary against Zed's built-in Markdown language. `navigator-lsp` itself
is licensed BUSL-1.1 and is downloaded at runtime, never bundled; this shim's own code is Apache-2.0 (see [LICENSE](LICENSE)).

## Developing

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

Install locally in Zed via the `zed: extensions` command palette action, then **Install Dev Extension**, pointing at
this checkout.
