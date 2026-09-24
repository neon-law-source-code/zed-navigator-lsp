# zed-navigator-lsp

A [Zed](https://zed.dev) extension shim that attaches
[`navigator-lsp`](https://github.com/neon-law-source-code/navigator/blob/main/docs/lsp/README.md) to Markdown
buffers.

This repository carries no Navigator source. At runtime the extension resolves the latest `navigator-lsp-<tag>-<platform>`
archive from [`neon-law-source-code/navigator`'s GitHub Releases](https://github.com/neon-law-source-code/navigator/releases),
downloads it, and launches the `navigator-lsp` binary against Zed's built-in Markdown language. `navigator-lsp` itself
and this shim are both licensed BUSL-1.1, the same licence as Navigator — see [LICENSE](LICENSE) and Navigator's
[NOTICE](https://github.com/neon-law-source-code/navigator/blob/main/NOTICE).

## Developing

```bash
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

Install locally in Zed via the `zed: extensions` command palette action, then **Install Dev Extension**, pointing at
this checkout.
