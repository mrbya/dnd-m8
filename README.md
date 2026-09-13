# D&D Mate

**D&D Mate** (a.k.a. D&D M8, DnD M8, ...) is an interactive, cross-platform charatcer sheet for Dungeons and Dragons-based games.

---

## Features

TBD

---

## Installation

TBD

### Platform-specific dependencies

**Linux**
```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows**
- WebView2 (ships with Windows 11; install from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) on Windows 10)
- Visual Studio C++ build tools

---

## Tech stack

TBD

---

## Development

### Prequisites

- [Rust toolchain](https://rustup.rs/) (stable, 1.77+)
- [Node.js](https://nodejs.org/) 20+ and `pnpm`
- [Tauri CLI v2](https://tauri.app/start/): `cargo install tauri-cli --version "^2"` or bootstrap using `just init` (see [Getting Started](#getting-started))

### Getting started

Requires `just` to bootstrap all tools and configuration

```bash
cargo install just
just init # setup repo and all required tools
```

Run in development:
```bash
just dev
```

Build for release:
```bash
just build
```

Before committing work if not using pre-commit hooks:
```bash
just pre-commit
```

To see all available commands:
```bash
just list
```

---

## Documentation

TBD

---

## License

Licensed under [MIT](LICENSE) license.
