# Rust Status Server

A lightweight HTTP server written in Rust that provides system information, health checks, and metrics endpoints. Inspired by [podinfo](https://github.com/stefanprodan/podinfo), this server features Nu-shell compatible structured data output and a modern terminal-themed HTML interface and Nu-shell structured data output.

## Features

- 🚀 Version information endpoint (`/version`)
- 💓 Health check endpoint (`/healthz`)
- 📊 Metrics endpoint (`/metrics`)
- 🎨 Terminal-themed HTML output
- 📋 JSON response support
- 📝 Nu-shell compatible structured data
- 📊 CSV export capabilities

## Quick Start
```
Clone the repository
git clone [[https://github.com/awdemos/demo-rust-server.git](https://github.com/awdemos/demo-rust-server.git)]
cd rust-status-server
cargo run
╭─────────┬───────────────────────╮
│ Status  │ Server Started        │
│ Address │ http://127.0.0.1:3000 │
╰─────────┴───────────────────────╯
```

Server will start at http://127.0.0.1:3000

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Dependencies

- tokio: Async runtime. This is for a simpler demo server using tokio. Might be removed later.
- nu-table: Terminal table formatting
- serde_json: JSON serialization
- rustc_version_runtime: Rust version information

## License

MIT License - See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Roadmap

- [ ] Implement feature parity with podinfo?
- [ ] Add Docker container support
- [ ] Add Kubernetes deployment examples
- [ ] More Nu-shell compatible output options