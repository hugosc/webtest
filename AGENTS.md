# 🤖 Guide for AI Agents - Leptos Learning Project

## About the User

- **Rust Level**: Beginner/Learning basics
- **Web Development Experience**: Limited
- **Learning Style**: Hands-on, practical examples
- **Goal**: Learn HTML and Rust together through building interactive web apps

## Project Overview

This is a **learning project** to teach HTML and Rust using the **Leptos framework**, a full-stack Rust web framework that compiles to WebAssembly (WASM).

### Tech Stack
- **Language**: Rust (beginner level)
- **Framework**: Leptos 0.7
- **Compilation Target**: WebAssembly (WASM)
- **Build Tool & Dev Server**: Trunk (live reload, builds, serves)

### Project Structure
```
webtest/
├── src/lib.rs              # Rust code (main app logic)
├── public/
│   ├── index.html          # HTML entry point with cache-busting
│   └── pkg/                # Generated WASM files (from wasm-pack)
├── Cargo.toml              # Dependencies
├── .cargo/config.toml      # WASM build config
└── Documentation files
```

## Key Learning Concepts

The user is learning:

1. **HTML Concepts**
   - Elements, attributes, event handlers
   - Styling with CSS
   - Structure and semantics

2. **Rust Concepts**
   - Macros (`#[component]`, `signal()`, `view!`)
   - Closures with `move` keyword
   - Pattern matching
   - Type inference

3. **Leptos-Specific**
   - **Signals**: Reactive state that auto-updates the UI
   - **Components**: Functions marked with `#[component]`
   - **View Macro**: HTML-like syntax (RSX) in Rust
   - **Event Handlers**: Closures that respond to user actions

## Development Workflow

**We now use [Trunk](https://trunkrs.dev/)** for building and serving the app. Trunk automatically rebuilds your Rust code to WASM and reloads the browser when you save changes.

### Quick Start

1. **Install Trunk** (if you haven't yet):
   ```bash
   cargo install trunk
   ```

2. **Start the Dev Server** (from the project root):
   ```bash
   trunk serve --open --public-url /public/
   ```
   - This will build your app, serve it locally, and open your browser.
   - Any changes to your Rust code will trigger an automatic rebuild and browser reload.

3. **Edit & Save**: Just edit `src/lib.rs` and save—Trunk handles the rest!

### Why Trunk?
- No more manual `wasm-pack` builds or copying files
- No need for Python's `http.server`
- Automatic cache busting and live reload
- Simpler, faster workflow for learning and experimenting

### Stopping the Server
Press `Ctrl+C` in the terminal

### Troubleshooting
- If you see errors, check the browser console (F12)
- Make sure Trunk is installed and you're running it from the project root
- If you change static files in `public/`, Trunk will also reload them

## Common Issues & Solutions

### Issue: Changes don't appear after save
**Solution**: 
1. Make sure Trunk is running (`trunk serve`)
2. Check browser console (F12) for errors
3. If needed, do a hard refresh: `Ctrl+Shift+R`

### Issue: "Port 8080 in use"
**Solution**: Use a different port
```bash
trunk serve --port 8081 --open --public-url /public/
```

### Issue: WASM module fails to load
**Solution**: 
1. Check browser console (F12) for error messages
2. Make sure Trunk is running and your build succeeded
3. Verify the `<script type="module">` in `index.html` uses `import init from "./pkg/webtest.js"; init();`

### Issue: Trunk not found
**Solution**: Install with `cargo install trunk`

## How to Help This User

### When Explaining Concepts
- Use **simple, concrete examples**
- Show **Leptos patterns** with actual code
- Relate to **HTML/JavaScript concepts** they might know
- Break down **Rust syntax** gently (closures, macros, etc.)

### When Debugging
1. **Check the browser console first** (F12 → Console)
2. **Verify files were actually rebuilt** (check timestamps in pkg/ vs public/pkg/)
3. **Ensure hard refresh happened** (Ctrl+Shift+R)
4. **Check for compilation errors** (`cargo check --target wasm32-unknown-unknown`)

### When Writing Code for Them
- Keep it **simple and readable**
- Add **comments** explaining Rust/Leptos-specific syntax
- Use **inline styles** first (easier to see in action)
- Start with **basic patterns** before advanced ones
- Show **both the what and the why**

### What NOT to Do
- Don't jump to advanced Leptos features (Resources, Actions, Server Functions)
- Don't use complex Rust patterns without explaining
- Don't assume knowledge of async/await, traits, or generics
- Don't skip the build/refresh steps in instructions

## Current App State

### What Works
- ✅ Basic counter with increment/decrement/reset buttons
- ✅ Responsive design with gradient background
- ✅ Signals for reactive state
- ✅ Event handling on buttons
- ✅ Development workflow (build → copy → refresh)
- ✅ Cache-busting on page load (timestamp-based)

### What to Build Next (Learning Progression)
1. **Add more buttons** (×2, +10, etc.) - learn event handlers
2. **Add conditional display** (show message when count > X) - learn logic
3. **Add text input** (set count manually) - learn two-way binding
4. **Create new components** - learn composition
5. **Build a real app** (todo list, calculator, etc.) - integrate knowledge

## Useful Documentation Links

- **Leptos Book**: https://book.leptos.dev/
- **Leptos API Docs**: https://docs.rs/leptos/
- **Rust Book**: https://doc.rust-lang.org/book/
- **MDN Web Docs**: https://developer.mozilla.org/ (HTML/CSS/JS concepts)

## Files in This Project

| File | Purpose | User Can Edit? |
|------|---------|---|
| `src/lib.rs` | Rust/Leptos app code | ✅ YES (main file to edit) |
| `public/index.html` | HTML entry point | ✅ YES (CSS styling) |
| `Cargo.toml` | Dependencies | ⚠️ CAREFULLY (they're set up) |
| `.cargo/config.toml` | WASM config | ❌ Leave as-is |
| `public/pkg/*` | Generated WASM | ❌ Auto-generated by Trunk, don't edit |

## Quick Reference: Leptos Patterns

### Creating State
```rust
let (count, set_count) = signal(0);
```

### Reading a Signal (in view)
```rust
view! { <p>"Count: " {count}</p> }
```

### Event Handler
```rust
let increment = move |_| set_count.update(|c| *c += 1);
view! { <button on:click=increment>"+"</button> }
```

### Component
```rust
#[component]
fn MyButton() -> impl IntoView {
    view! { <button>"Click me"</button> }
}
```

### Building & Serving (with Trunk)
```bash
trunk serve --open --public-url /public/
```
- Edit your code and Trunk will rebuild and reload automatically!

## Communication Tips

- **Be encouraging**: Learning Rust + Web Dev is challenging!
- **Break it down**: Small, focused changes instead of large rewrites
- **Show the "why"**: Explain not just what, but why it works this way
- **Celebrate progress**: Each working feature is a win
- **Give next steps**: Point them toward the next learning milestone

---

**Remember**: This is a learning project. The goal is understanding, not just making it work. Take time to explain concepts and let them experiment! 🦀