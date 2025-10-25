# 🦀 Leptos Learning Project

Welcome to your journey of learning **Rust** and **HTML** through **Leptos**, a full-stack web framework that compiles Rust to WebAssembly!

## What is Leptos?

Leptos is a modern web framework that lets you write interactive web applications entirely in Rust. Your code compiles to WebAssembly (WASM) that runs in the browser, giving you the performance and safety of Rust combined with the interactivity of web applications.

### Key Features

- **Fine-grained Reactivity**: Signals automatically track dependencies and update only what changed
- **Type Safety**: Catch bugs at compile time with Rust's type system
- **WebAssembly**: Compile to WASM for blazing fast performance
- **Full-Stack Potential**: Write both frontend and backend in Rust

## Project Structure

```
webtest/
├── Cargo.toml              # Project configuration and dependencies
├── .cargo/
│   └── config.toml        # Cargo settings for WASM compilation
├── src/
│   └── lib.rs             # Rust source code (the component library)
├── public/
│   └── index.html         # HTML entry point
├── pkg/                   # Generated WebAssembly files (after build)
│   ├── webtest.js         # JavaScript glue code
│   ├── webtest_bg.wasm    # Compiled WebAssembly binary
│   └── ...
└── README.md              # This file
```

## Getting Started

### Prerequisites

You have Rust basics installed. Let's verify everything is set up:

```bash
# Check Rust version (should be recent)
rustc --version

# Check if wasm32 target is installed
rustc --print sysroot

# Verify wasm-pack is installed
wasm-pack --version
```

### Building the Project

The project is already set up and built! Here's how it works:

1. **Rust code** in `src/lib.rs` defines the UI components
2. **Build to WASM** using `wasm-pack`:
   ```bash
   wasm-pack build --target web --release
   ```
3. **Output** goes to the `pkg/` directory
4. **HTML** loads the WASM and runs it

### Running the Project

The web server should already be running on `http://localhost:8000`. If not:

```bash
cd public
python3 -m http.server 8000
```

Then open your browser to `http://localhost:8000`

## Understanding Your First App

### The HTML (`public/index.html`)

The HTML file is minimal:

```html
<div id="app"></div>
<script type="module">
  import init from "../pkg/webtest.js";
  init();
</script>
```

This:
1. Creates a container `<div id="app">` where Leptos will render
2. Imports the compiled JavaScript glue code
3. Calls `init()` to start the WebAssembly module

The rest is CSS styling to make your app look beautiful!

### The Rust Code (`src/lib.rs`)

Let's break down the key concepts:

#### Signals - Reactive State

```rust
let (count, set_count) = signal(0);
```

A **signal** is a reactive value. When it changes, Leptos automatically updates the UI.

- `signal(0)` creates a signal starting at `0`
- It returns a tuple: `(ReadSignal, WriteSignal)`
- `count` is the readable version (you get values from it)
- `set_count` is the writable version (you update it)

**Signals are Copy!** You can easily pass them into closures.

#### Components - Reusable UI

```rust
#[component]
fn Counter() -> impl IntoView {
    // Component code here
}
```

The `#[component]` macro turns a function into a Leptos component that returns something renderable.

#### View Macro - Declarative UI

```rust
view! {
    <button on:click=increment>"Click me"</button>
}
```

The `view!` macro lets you write HTML-like syntax (called RSX) directly in Rust!

- Regular HTML elements work: `<div>`, `<button>`, `<p>`, etc.
- Text goes in quotes: `"Hello"`
- Rust values use curly braces: `{count}`
- Event handlers: `on:click=`, `on:input=`, etc.
- Classes: `class="name"` or dynamic: `class:active=move || count.get() > 5`

#### Event Handlers - Responding to User Actions

```rust
let increment = move |_| set_count.update(|c| *c += 1);
```

This creates a closure that:
- Uses `move` to capture `set_count` by moving it in (it's Copy, so cheap)
- Takes an event parameter (we ignore it with `_`)
- Updates the signal: `set_count.update(|c| *c += 1)` increments it

Then you use it: `on:click=increment`

#### Mounting - Starting the App

```rust
#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <App /> })
}
```

This:
- Marks the function as the WASM entry point
- Mounts the `App` component to the document body (the entire page)

## How Signals Work

This is the heart of Leptos's reactivity. Here's a complete example:

```rust
#[component]
fn Example() -> impl IntoView {
    // Create a signal (Copy type)
    let (count, set_count) = signal(0);
    
    // Read the value
    let current = count.get();  // Gets the current value
    
    // Update the value
    let increment = move |_| {
        // Method 1: Replace the entire value
        set_count.set(count.get() + 1);
        
        // Method 2: Transform the current value
        set_count.update(|c| *c += 1);  // Usually preferred
    };
    
    // Use in the view
    view! {
        <p>"Count: " {count}</p>  // Reactive - updates automatically!
        <button on:click=increment>"+"</button>
    }
}
```

**Key insight**: When you put `{count}` in the view, Leptos creates a **dependency** on that signal. When `count` changes, only that text updates—not the entire component!

## Your Current App

Your starter app is a counter with three buttons:

1. **Increment (+1)**: Adds 1 to the count
2. **Decrement (-1)**: Subtracts 1 from the count
3. **Reset**: Sets count back to 0

Each button updates the signal, which automatically updates the display. Try clicking the buttons!

## Challenges to Learn

Now that you understand the basics, try these challenges:

### Challenge 1: Change the Starting Value
**Goal**: Make the counter start at 50 instead of 0

Look at: `signal(0)` in the `Counter` component. Change the `0` to `50`.

**Why**: Understanding how to initialize signals.

### Challenge 2: Add a "Triple" Button
**Goal**: Add a fourth button that multiplies the count by 3

```rust
let triple = move |_| set_count.update(|c| *c *= 3);
```

Then add it to the button group in the view:
```rust
<button on:click=triple class="btn btn-triple">"×3"</button>
```

And add CSS styling for `.btn-triple` (pick a color!).

**Why**: Practice creating event handlers and working with the view macro.

### Challenge 3: Display a Message
**Goal**: Show different messages based on the count

Try adding this to your view:
```rust
<p>
  {move || {
    if count.get() > 10 {
      "Count is high!"
    } else if count.get() < 0 {
      "Count is negative!"
    } else {
      "Count is normal"
    }
  }}
</p>
```

**Why**: Learn that functions in views are tracked for reactivity too.

### Challenge 4: Add a Text Input
**Goal**: Let users type in a new count value

```rust
let (input, set_input) = signal(String::new());
```

Add to the view:
```rust
<input
    type="text"
    prop:value=input
    on:input=move |ev| set_input.set(event_target_value(&ev))
/>
<button on:click=move |_| {
    if let Ok(n) = input.get().parse::<i32>() {
        set_count.set(n);
    }
}>
    "Set Count"
</button>
```

Then import `event_target_value`:
```rust
use leptos::ev::*;
```

**Why**: Learn about two-way binding and working with form inputs.

### Challenge 5: Create a New Component
**Goal**: Extract the button group into a reusable component

```rust
#[component]
fn ButtonGroup(
    count: ReadSignal<i32>,
    on_increment: impl Fn() + 'static,
    on_decrement: impl Fn() + 'static,
    on_reset: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <div class="button-group">
            <button on:click=move |_| on_decrement() class="btn btn-decrease">"-1"</button>
            <button on:click=move |_| on_increment() class="btn btn-increase">"+1"</button>
            <button on:click=move |_| on_reset() class="btn btn-reset">"Reset"</button>
        </div>
    }
}
```

Then use it in `Counter`:
```rust
<ButtonGroup
    count=count
    on_increment=move || set_count.update(|c| *c += 1)
    on_decrement=move || set_count.update(|c| *c -= 1)
    on_reset=move || set_count.set(0)
/>
```

**Why**: Understand component composition and prop passing.

## Common Patterns

### Reading a Signal

```rust
// Get the value and track reactively
let value = count.get();

// Get value without tracking (advanced)
let value = count.get_untracked();

// Use with a closure (common in views)
count.with(|val| println!("Count: {}", val))
```

### Writing a Signal

```rust
// Set to a specific value
set_count.set(0);

// Transform the current value
set_count.update(|c| *c += 1);

// Direct write (less common)
*set_count.write() = 0;
```

### Closures in Views

When you put a closure in a view, Leptos tracks what signals it reads and re-runs it when they change:

```rust
view! {
    <p>{move || count.get() * 2}</p>  // Re-runs when count changes
}
```

The `move` keyword is important—it captures variables by moving them into the closure.

### Conditional Rendering (Simple)

```rust
view! {
    <p>
      {move || {
        if count.get() > 5 {
          "High"
        } else {
          "Low"
        }
      }}
    </p>
}
```

### Conditional Rendering (Advanced)

```rust
use leptos::prelude::*;

view! {
    <Show
        when=move || count.get() > 5
        fallback=|| view! { <p>"Count is 5 or less"</p> }
    >
        <p>"Count is greater than 5!"</p>
    </Show>
}
```

## Debugging Tips

### Check the Browser Console

Press `F12` (or right-click → Inspect) to open DevTools. Any Leptos errors will appear in the Console tab.

### Add Logging

```rust
use leptos::logging::log;

log!("Count: {}", count.get());
```

Messages appear in the browser console.

### Rebuild on Changes

After editing `src/lib.rs`, rebuild:

```bash
wasm-pack build --target web --release
```

Then refresh your browser (`F5` or `Ctrl+R`).

## Next Steps to Master Leptos

1. **Memos** - Cached computations that only recalculate when dependencies change
2. **Effects** - Side effects that run when signals change (like logging or API calls)
3. **Resources** - Async data fetching
4. **Context API** - Share data across the entire component tree
5. **Routing** - Build multi-page apps with `leptos_router`
6. **Server Functions** - Call backend Rust code from the frontend (full-stack!)

## Useful Resources

- **Official Docs**: https://docs.rs/leptos/latest/leptos/
- **Book**: https://book.leptos.dev/
- **GitHub**: https://github.com/leptos-rs/leptos
- **Discord**: Join the Leptos community for help

## Troubleshooting

### App doesn't load
1. Check browser console (`F12`)
2. Make sure `wasm-pack build --target web --release` ran successfully
3. Verify `http://localhost:8000` loads and you see no 404 errors

### "Error: Unexpected token" in console
This usually means the WASM files aren't being served. Check that:
- The `pkg/` directory exists
- The HTML path to the script is correct: `import init from "../pkg/webtest.js"`

### Compilation errors in Rust
Read the error messages carefully! Rust's compiler is very helpful. Common issues:
- Missing `move` keyword in closures
- Type mismatches
- Incorrect event handler signatures

## The Learning Path Ahead

```
1. ✅ Signals & Basic Reactivity (you are here)
   ↓
2. Event Handling & DOM Updates
   ↓
3. Component Composition & Props
   ↓
4. Derived State (Memos)
   ↓
5. Side Effects (Effects)
   ↓
6. Async Data (Resources)
   ↓
7. Full-Stack Rust (Server Functions)
   ↓
8. Production Apps
```

## Key Takeaways

- **Signals** are reactive values that automatically update the UI
- **Components** are functions that return views
- **The view! macro** lets you write HTML in Rust
- **Closures** in views automatically track signal dependencies
- **Everything is Rust** - you get all the safety and performance benefits

Now go build something amazing! 🚀

---

**Happy coding!** Remember: the best way to learn is by doing. Start with Challenge 1 and work your way up. Don't be afraid to experiment!