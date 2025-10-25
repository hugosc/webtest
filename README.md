# 🦀 Leptos Learning Project

A hands-on Rust + Leptos web app for learning HTML, Rust, and modern WASM workflows.

## Project Overview

- **Framework:** [Leptos](https://leptos.dev/) (Rust, WASM)
- **Build Tool:** [Trunk](https://trunkrs.dev/)
- **Goal:** Simple, interactive web apps for learning Rust and web development

## Quick Start

1. **Install Trunk** (if you haven't yet):
   ```bash
   cargo install trunk
   ```

2. **Run the Dev Server**:
   ```bash
   trunk serve --open --public-url /public/
   ```
   - Edit `src/lib.rs` and save—Trunk rebuilds and reloads automatically.

3. **Stop the server:** Press `Ctrl+C` in the terminal.

## Project Structure

```
webtest/
├── src/lib.rs         # Main Rust app code (edit here)
├── public/index.html  # HTML entry point
├── Cargo.toml         # Rust dependencies
├── .cargo/            # WASM build config
├── dist/              # Trunk build output (auto-generated)
├── pkg/               # WASM output (auto-generated)
├── .gitignore         # Ignore rules (see below)
├── GETTING_STARTED.md # Learning guide & challenges
└── README.md          # This file (minimal)
```

## .gitignore

This project uses a comprehensive `.gitignore` to keep your repo clean:

- Ignores build artifacts (`/target/`, `/dist/`, `/pkg/`, etc.)
- Ignores editor/OS files, logs, and temp files
- Ignores project-specific and AI agent config files (`AGENTS.md`, `allat`, etc.)
- Files like `a.out`, `Cargo.lock`, and others are now untracked

## Learning & Documentation

- **Learning guide:** See [`GETTING_STARTED.md`](GETTING_STARTED.md)
- **Leptos Book:** https://book.leptos.dev/
- **API Docs:** https://docs.rs/leptos/

---

**Happy coding!**  
_Edit `src/lib.rs` and experiment. For hands-on learning, follow the steps in `GETTING_STARTED.md`._