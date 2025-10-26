# LEARNING.md - Quick Reference Guide for Leptos Counter App

## File Overview

This project teaches you how to build reactive web UIs in Rust that compile to WebAssembly (WASM).

**Start here:** Read `src/lib.rs` from top to bottom. It's heavily documented with inline comments explaining every concept.

---

## Core Concepts at a Glance

### 1. Components
Functions that return UI. Marked with `#[component]` attribute.

```rust
#[component]
fn MyButton(label: String) -> impl IntoView {
    view! { <button>{label}</button> }
}
```

Use them in views:
```rust
view! { <MyButton label="Click me" /> }
```

### 2. Signals (Reactive State)
State that triggers automatic UI updates when changed.

```rust
// Create a signal
let (count, set_count) = signal(0);

// Read the current value
let current = count.get();

// Update the value
set_count.set(42);              // Replace
set_count.update(|c| *c += 1);  // Modify
```

When you call `set_count.set()` or `set_count.update()`, Leptos automatically re-renders all components that depend on that signal.

### 3. The view! Macro
HTML-like syntax for rendering. Looks like HTML but it's Rust code.

```rust
view! {
    <div class="container">
        <h1>"Hello World"</h1>
        <button on:click=my_handler>
            "Click me"
        </button>
    </div>
}
```

### 4. Reactivity Pattern (THE CRITICAL PATTERN)
Always wrap signal access in closures inside views:

```rust
// ✅ CORRECT - Signal access in a closure
view! {
    <p>{move || signal.get()}</p>
}

// ❌ WRONG - Signal access outside a closure
let value = signal.get();  // Gets frozen value, no reactivity
view! {
    <p>{value}</p>
}
```

**Why?** Leptos needs to track dependencies. Closures let Leptos say "this view depends on this signal". When the signal changes, the closure re-runs automatically.

### 5. Event Handlers
Closures that respond to user interactions.

```rust
let handle_click = move |event: MouseEvent| {
    set_count.update(|c| *c += 1);
};

view! {
    <button on:click=handle_click>
        "Increment"
    </button>
}
```

The `move` keyword is important - it captures variables from the surrounding scope.

### 6. Component Composition
Build complex UIs from simple components.

```
App
├── EvilBackground (layout)
│   └── Counter (feature)
│       ├── CounterDisplay (presentational)
│       ├── CounterMessage (presentational)
│       └── CounterButtons (presentational)
│           ├── EvilButton
│           ├── EvilButton
│           └── EvilButton (reused 5 times)
```

Each component is simple and focused. Complex UIs emerge from composing them together.

---

## The Counter App Architecture

### Components in src/lib.rs

1. **EvilBackground** - Full-screen layout with gradient background
   - Takes `children: Children` - renders whatever is passed inside
   - Uses flexbox to center content

2. **EvilButton** - Reusable button component
   - Props: `label` (text), `on_click` (handler function)
   - Styled with inline CSS using `format!` macro
   - Used 5 times in CounterButtons

3. **CounterDisplay** - Shows the current count
   - Takes a closure that returns the current count
   - Displays in large red text
   - Updates reactively when count changes

4. **CounterMessage** - Shows context-dependent messages
   - Takes the `count` signal directly
   - Demonstrates conditional rendering pattern
   - Message changes based on count value:
     - High (>50): "🔥 Count is HIGH!"
     - Negative (<0): "❄️ Count is NEGATIVE!"
     - Zero (==0): "😴 Count is ZERO!"
     - Normal (1-49): "Count is normal"

5. **CounterButtons** - Renders all action buttons
   - Takes 5 event handler callbacks
   - Each button calls a different handler
   - Shows component composition and reusability

6. **Counter** - Main feature component
   - Manages the count signal
   - Creates all event handlers
   - Composes Display, Message, and Buttons

7. **App** - Root component
   - Combines EvilBackground (layout) with Counter (feature)
   - Shows app structure pattern

---

## Key Patterns to Remember

### Pattern 1: Reactive Closures in Views
```rust
// Always use move || for signal access in views
view! {
    <p>{move || count.get()}</p>  // ✅ Reactive
}
```

### Pattern 2: Conditional Rendering
Don't return different views from if/else. Compute values, render one structure:

```rust
view! {
    {
        // Step 1: Compute the value using Rust if/else
        let message = if count > 50 { "High" } else { "Low" };
        
        // Step 2: Return ONE view structure with dynamic content
        move || {
            view! {
                <p>{message}</p>
            }
        }
    }
}
```

### Pattern 3: Event Handlers
```rust
let handler = move |_event: MouseEvent| {
    // The `_` means we ignore the event object
    // We just want to know the button was clicked
    set_count.update(|c| *c += 1);
};

view! { <button on:click=handler>"Click"</button> }
```

### Pattern 4: Component Props
```rust
#[component]
fn MyComponent(
    label: String,
    value: i32,
    on_click: Box<dyn Fn(MouseEvent) + 'static>,
) -> impl IntoView {
    // Use props in the component
}

// Use the component
view! {
    <MyComponent
        label="Test"
        value=42
        on_click=Box::new(handler)
    />
}
```

---

## Development Workflow

### Start the dev server
```bash
cd webtest
trunk serve --open
```

This will:
- Compile Rust to WebAssembly
- Start a local server (usually http://localhost:8080)
- Open your browser
- Auto-reload when you change code

### Build for production
```bash
trunk build --release
```

Creates optimized output in `dist/`

### Check for errors
```bash
cargo build --target=wasm32-unknown-unknown
```

### Clean rebuild
```bash
rm -rf dist target && trunk build
```

---

## Common Mistakes & How to Fix Them

### Mistake 1: Signal access outside a closure
```rust
// ❌ WRONG
let value = signal.get();
view! { <p>{value}</p> }  // Won't update when signal changes

// ✅ CORRECT
view! { <p>{move || signal.get()}</p> }  // Updates automatically
```

### Mistake 2: Returning different views from if/else
```rust
// ❌ WRONG - Type mismatch
view! {
    if count > 50 {
        <p>"High"</p>      // Type A
    } else {
        <p>"Low"</p>       // Type B - different!
    }
}

// ✅ CORRECT - Compute value, render once
view! {
    {
        let message = if count > 50 { "High" } else { "Low" };
        move || {
            view! { <p>{message}</p> }
        }
    }
}
```

### Mistake 3: Forgetting `move` in closures
```rust
// ❌ WRONG - Lifetime errors
let handler = |_| {
    set_count.update(|c| *c += 1);  // Borrow checker complains
};

// ✅ CORRECT
let handler = move |_| {  // Capture by moving
    set_count.update(|c| *c += 1);
};
```

### Mistake 4: Passing signal value instead of signal
```rust
// ❌ WRONG
let count_val = count.get();
view! { <CounterMessage count=count_val /> }  // Frozen value

// ✅ CORRECT
view! { <CounterMessage count=count /> }  // Pass signal, gets updates
```

---

## Next Steps to Extend the App

### Difficulty: Easy
1. **Change the step size** - Make buttons increment by 5 instead of 1
2. **Add more buttons** - Add a "x10" button, "÷10" button
3. **Change the theme** - Use different colors in the theme module

### Difficulty: Medium
4. **Add localStorage** - Save count to browser storage, restore on refresh
5. **Add an input field** - Let users type a number to set the count directly
6. **Add operation history** - Display last 10 operations ("+1", "*2", etc.)

### Difficulty: Hard
7. **Multi-counter** - Render multiple independent counters
8. **Preset buttons** - Save/load favorite values
9. **Extract CSS** - Move inline styles to separate CSS file
10. **Add animations** - Animate count changes

---

## Important Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | **Main file - read this first!** Heavily documented components |
| `index.html` | HTML loader (empty `<body>`, Trunk injects app) |
| `Cargo.toml` | Rust dependencies and configuration |
| `Trunk.toml` | Leptos/Trunk build configuration |
| `AGENTS.md` | Notes for AI assistants (terminal command warnings) |
| `LEARNING.md` | This file - quick reference |

---

## Resources

- **Leptos Official Docs:** https://leptos.dev
- **Trunk Documentation:** https://trunkrs.io
- **Rust Book:** https://doc.rust-lang.org/book/
- **MDN Web Docs:** https://developer.mozilla.org

---

## Summary

You now have a fully working Leptos counter app that demonstrates:
- ✓ Component-based architecture
- ✓ Reactive state with signals
- ✓ Event handling
- ✓ Conditional rendering
- ✓ Component composition
- ✓ Styling in Rust

The best way to learn is by modifying the code. Change colors, add buttons, modify logic, and see what happens. The app is built to teach you through doing.

Read `src/lib.rs` whenever you want to understand how something works. Every section is documented.

Happy coding! 🚀