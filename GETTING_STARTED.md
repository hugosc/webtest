# Getting Started with Leptos 🦀

Welcome! You've just set up a complete Leptos project for learning HTML and Rust through interactive web development.

## What You Have

Your project structure is:

```
webtest/
├── src/lib.rs                 # Your Rust code (the app logic)
├── public/index.html          # HTML page + styling
├── pkg/                       # Compiled WebAssembly (generated)
│   ├── webtest.js            # JavaScript glue code
│   └── webtest_bg.wasm       # Compiled Rust → WebAssembly
├── .cargo/config.toml         # Cargo WebAssembly settings
├── Cargo.toml                 # Project dependencies
├── README.md                  # Comprehensive guide
├── QUICKSTART.md              # Quick reference
└── GETTING_STARTED.md         # This file
```

## The Big Picture

Here's how your app works:

1. **You write Rust** in `src/lib.rs`
2. **You build to WebAssembly** using `wasm-pack build --target web --release`
3. **The browser loads the WASM** via JavaScript in `public/index.html`
4. **Your reactive UI runs** compiled to blazing-fast machine code in the browser

## Quick Start (5 Minutes)

### 1. Start the Web Server

```bash
cd webtest/public
python3 -m http.server 8000
```

### 2. Open Your Browser

Visit: **http://localhost:8000**

You'll see a purple gradient background with a counter that has three buttons.

### 3. Click the Buttons!

Watch the count change in real-time. That's Leptos reactivity! 🎉

### 4. Make a Change

Edit `src/lib.rs` and change:
```rust
let (count, set_count) = signal(0);
```

to:
```rust
let (count, set_count) = signal(100);
```

### 5. Rebuild

```bash
wasm-pack build --target web --release
```

### 6. Refresh Your Browser

Press F5 to see the counter start at 100!

## Understanding Leptos in 3 Concepts

### 1. Signals = Reactive State

```rust
let (count, set_count) = signal(0);
```

A signal is a value that automatically triggers UI updates when changed. Think of it as a "smart variable."

- `count` - read the value
- `set_count` - change the value
- When you change it, the UI updates automatically ✨

### 2. Components = Reusable UI Pieces

```rust
#[component]
fn Counter() -> impl IntoView {
    // UI code here
}
```

A component is a function that returns rendered HTML. The `#[component]` macro makes it work with Leptos.

### 3. View Macro = HTML in Rust

```rust
view! {
    <button on:click=increment>"Click me"</button>
}
```

The `view!` macro lets you write HTML directly in Rust. It's like JSX but in Rust!

## The App You Have

Your starter app is a **Counter** with three buttons:

- **+1 Button**: Increases count by 1
- **-1 Button**: Decreases count by 1  
- **Reset Button**: Sets count back to 0

The display shows "Current count: X" where X updates instantly.

## What You're Learning

By building with Leptos, you're learning:

### HTML Concepts
- Structure with tags (`<div>`, `<button>`, `<span>`)
- Attributes (`class`, `id`, `on:click`)
- Event handling (`on:click`, `on:input`)
- Styling with CSS

### Rust Concepts
- Macros (`#[component]`, `signal()`, `view!`)
- Closures (`|_| { ... }`)
- Pattern matching (`move` keyword)
- Type inference
- Ownership and borrowing

### Web Development Concepts
- Reactive programming (UI updates automatically)
- Event-driven architecture (respond to user actions)
- State management (signals)
- Component composition (build UIs from pieces)

## Your Learning Path

### Level 1: Get Comfortable ⭐
- [ ] Start the server and see the app running
- [ ] Click all the buttons and understand what they do
- [ ] Change the starting value (challenge!)
- [ ] Understand how signals work
- [ ] Read through the code and understand each part

### Level 2: Add Features ⭐⭐
- [ ] Add a "×2" button that multiplies the count
- [ ] Add a "+5" button
- [ ] Change button colors in CSS
- [ ] Add a message that shows when count is above/below certain values
- [ ] Create your own event handler

### Level 3: Build Something Real ⭐⭐⭐
- [ ] Build a simple calculator
- [ ] Create a todo list app
- [ ] Make a number guessing game
- [ ] Build a form with multiple inputs
- [ ] Create reusable components

### Level 4: Advanced Topics ⭐⭐⭐⭐
- [ ] Learn about Memos (cached computations)
- [ ] Learn about Effects (side effects like logging)
- [ ] Learn about Resources (async data)
- [ ] Learn about Context (sharing data across components)
- [ ] Build full-stack apps with server functions

## Important Files to Know

### `src/lib.rs`
This is where you write your Rust code. Every component you create goes here. Start by modifying the existing `Counter` component.

### `public/index.html`
This is the HTML page that loads your app. It contains:
- A `<div id="app">` where Leptos renders your UI
- A `<script>` that imports and initializes your WebAssembly
- CSS styling to make everything look nice

Don't modify the JavaScript parts unless you know what you're doing. You can modify the CSS to style your app!

### `Cargo.toml`
This file lists your project dependencies. Leptos and other libraries are listed here. You can add more dependencies if you need them.

## Development Workflow

This is your typical workflow:

1. **Edit** `src/lib.rs` with code changes
2. **Rebuild** with `wasm-pack build --target web --release`
3. **Refresh** your browser to see changes (F5)
4. **Debug** using browser DevTools (F12)
5. **Repeat!**

## Rebuilding Your App

Every time you change `src/lib.rs`, you need to rebuild:

```bash
wasm-pack build --target web --release
```

This compiles your Rust code to WebAssembly and generates JavaScript glue code in the `pkg/` directory.

**Pro tip**: You can make a shell script to rebuild automatically:

```bash
#!/bin/bash
while true; do
    wasm-pack build --target web --release
    sleep 2
done
```

Or use `cargo watch` if you have it installed:

```bash
cargo watch -x "build --target wasm32-unknown-unknown --release"
```

## Browser DevTools (Your Best Friend)

Press `F12` to open browser DevTools:

- **Console tab**: See error messages and logs
- **Elements tab**: Inspect the HTML Leptos generated
- **Network tab**: See WASM and JS files loading
- **Application tab**: See LocalStorage, cookies, etc.

## Common Questions

### Q: Why WebAssembly?
A: WebAssembly (WASM) is much faster than JavaScript. Your Rust code compiles to machine-like code that runs super fast in the browser.

### Q: Why write in Rust instead of JavaScript?
A: Rust catches bugs at compile-time instead of runtime. You get better performance and fewer surprises. Plus, you get the same type safety on the frontend as the backend!

### Q: Can I use NPM packages?
A: Yes! You can use JavaScript libraries through Leptos. But most common needs have Rust equivalents.

### Q: Can I deploy this?
A: Yes! Build with `wasm-pack build --target web --release`, then serve the files on any static hosting (Netlify, Vercel, GitHub Pages, etc.).

### Q: Is Leptos used by real companies?
A: Yes! Many companies use Leptos for production applications. The ecosystem is growing fast.

## Troubleshooting

### "App won't load in browser"
1. Check DevTools Console (F12)
2. Make sure `pkg/` directory exists
3. Verify the HTML path to the script: `../pkg/webtest.js`
4. Check web server is running: `python3 -m http.server 8000`

### "Changes don't appear"
1. Did you rebuild? `wasm-pack build --target web --release`
2. Did you refresh browser? F5 (or Ctrl+R)
3. Try hard refresh: Ctrl+Shift+R

### "Compilation error in Rust"
Read the error message! Rust's compiler is very helpful. Common issues:
- Missing `move` keyword in closures
- Incorrect signal types
- Missing imports

### "Port 8000 already in use"
Use a different port:
```bash
cd webtest/public
python3 -m http.server 8001
```

Then visit `http://localhost:8001`

## Documentation

- **QUICKSTART.md** - Quick reference for common tasks
- **README.md** - Comprehensive guide with examples and challenges
- **Leptos Book**: https://book.leptos.dev
- **API Docs**: https://docs.rs/leptos/

## Next Steps

1. **Read QUICKSTART.md** for a quick reference
2. **Try Challenge 1**: Change the starting value to 50
3. **Try Challenge 2**: Add a "×2" button
4. **Read README.md** for more in-depth learning
5. **Build something of your own!**

## Key Takeaways

- ✅ Signals make your UI reactive automatically
- ✅ Components organize your code into reusable pieces
- ✅ The view! macro lets you write HTML in Rust
- ✅ Rust catches bugs at compile-time, not runtime
- ✅ WebAssembly runs super fast in the browser
- ✅ Full-stack Rust development is possible!

## You're Ready!

You have everything set up. Your app is running. The only thing left is to start coding and learning.

**Go make something awesome!** 🚀

---

Remember: the best way to learn is by doing. Start with small changes, experiment, break things, and fix them. That's how developers learn.

Happy coding! 🦀