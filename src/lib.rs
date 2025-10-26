/// ============================================================================
/// LEPTOS COUNTER APP - HEAVILY DOCUMENTED FOR LEARNING
/// ============================================================================
///
/// This file demonstrates core Leptos concepts for building reactive web UIs
/// in Rust that compile to WebAssembly (WASM).
///
/// KEY CONCEPTS YOU'LL LEARN:
/// 1. Components - reusable UI building blocks
/// 2. Signals - reactive state that triggers UI updates
/// 3. Views - Leptos macro for rendering HTML-like syntax
/// 4. Event handlers - responding to user interactions
/// 5. Reactivity - automatic UI updates when state changes
/// 6. Component composition - building complex UIs from simple pieces
///
/// READ THIS FILE FROM TOP TO BOTTOM to understand how everything connects.
/// Each section builds on previous concepts.
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// ============================================================================
// SECTION 1: CONFIGURATION & THEME CONSTANTS
// ============================================================================
//
// WHY: Centralizing color values makes it easy to change the theme globally
//      without editing multiple components.
//
// HOW: We define a `theme` module with color constants. Each component
//      can reference these colors instead of hardcoding hex values.
//
// LEARNING: This is called the "DRY principle" (Don't Repeat Yourself).
//           If you need to change the red color, you change it in ONE place,
//           and ALL components using `theme::EVIL_RED` automatically get the update.

mod theme {
    /// Dark grey used for backgrounds and contrast
    pub const DARK_GREY: &str = "#232323";

    /// Deep red - the primary brand color
    pub const EVIL_RED: &str = "#8b0000";

    /// Bright red - used for highlights and accents
    pub const BRIGHT_RED: &str = "#ff1744";

    /// Background color for cards/containers
    pub const CARD_BG: &str = "#18141a";

    /// Muted red/grey for secondary text
    pub const TEXT_MUTED: &str = "#e57373";
}

// ============================================================================
// SECTION 2: LAYOUT COMPONENTS
// ============================================================================
//
// WHAT: Layout components handle the overall page structure and styling.
//       They don't manage state, they just render HTML with styling.
//
// WHY: Separating layout from logic makes components reusable and testable.
//
// KEY CONCEPT: The `Children` type lets this component wrap ANY content.
//              This is how you create container/layout components.

/// EvilBackground - A full-screen layout component with gradient background
///
/// WHAT IT DOES:
/// - Creates a full-screen div with a gradient background
/// - Centers its children in the middle of the screen
/// - Uses flexbox for centering (align-items, justify-content)
///
/// HOW IT WORKS:
/// 1. Takes a `children: Children` parameter
/// 2. `children` is a Leptos type that represents nested components/HTML
/// 3. Inside the view!, call `{children()}` to render whatever was passed in
///
/// EXAMPLE USAGE:
/// ```rust
/// view! {
///     <EvilBackground>
///         <h1>"Hello World"</h1>      // This becomes a child
///         <Counter />                  // This too
///     </EvilBackground>
/// }
/// ```
///
/// LEARNING: This pattern (wrapping children) is how you build flexible layouts.
///           Think of it like HTML's <body> or <div> elements that can wrap anything.
#[component]
fn EvilBackground(children: Children) -> impl IntoView {
    // Create a CSS gradient string that transitions from DARK_GREY to EVIL_RED
    // The 135deg angle creates a diagonal gradient (top-left to bottom-right)
    let bg_gradient = format!(
        "linear-gradient(135deg, {} 0%, {} 100%)",
        theme::DARK_GREY,
        theme::EVIL_RED
    );

    // The view! macro returns HTML-like syntax that Leptos converts to actual DOM elements
    // We'll explain this in detail below.
    view! {
        // A full-screen div with flexbox centering
        <div style=format!(
            // min-height/min-width: Make it at least the full screen size
            // background: Apply the gradient we created above
            // display: flex: Enable flexbox layout
            // justify-content: center: Center horizontally
            // align-items: center: Center vertically
            // padding: 20px: Add some breathing room on mobile
            "min-height: 100vh; min-width: 100vw; background: {}; display: flex; justify-content: center; align-items: center; padding: 20px;",
            bg_gradient
        )>
            // Render whatever was passed as children
            // This could be <Counter />, <h1>, etc.
            {children()}
        </div>
    }
}

// ============================================================================
// SECTION 3: REUSABLE UI COMPONENTS
// ============================================================================
//
// WHAT: These are small, self-contained components that handle ONE thing well.
//
// WHY: Small components are easier to understand, test, and reuse.
//      If you need a button in 10 places, you define it once and use it 10 times.
//
// KEY CONCEPT: Components can take "props" (properties/parameters) that control
//              how they look and behave. This makes them configurable.

/// EvilButton - A reusable button with consistent styling
///
/// WHAT IT DOES:
/// - Renders a styled button with the evil red theme
/// - Takes two props: label (text) and on_click (event handler)
/// - When clicked, calls the on_click handler
///
/// PROPS EXPLANATION:
/// - `label`: The text displayed on the button
///   - `#[prop(into)]` means you can pass a String, &str, or anything convertible to String
/// - `on_click`: A boxed function that handles click events
///   - `Box<dyn Fn(...)>` means "a pointer to any function that takes a MouseEvent"
///   - The `'static` lifetime means the function lives for the entire app duration
///
/// WHY USE PROPS:
/// Instead of hardcoding button text and handlers, we parameterize them.
/// This lets us use ONE button component for many different buttons:
/// ```rust
/// <EvilButton label="Increment" on_click=Box::new(handle_increment) />
/// <EvilButton label="Decrement" on_click=Box::new(handle_decrement) />
/// <EvilButton label="Reset" on_click=Box::new(handle_reset) />
/// ```
///
/// LEARNING: The more configurable your components, the more you'll reuse them.
#[component]
fn EvilButton(
    /// The text to display on the button
    #[prop(into)]
    label: String,

    /// The function to call when the button is clicked
    /// Takes a MouseEvent (browser event) as a parameter
    on_click: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
) -> impl IntoView {
    // Build the CSS style string that makes the button look "evil"
    let button_style = format!(
        // padding: Space inside the button (text to edge)
        // margin: Space outside the button (button to other elements)
        // background: Gradient from EVIL_RED to DARK_GREY
        // color: White text
        // border: None (we don't want a browser default border)
        // border-radius: Rounded corners (6px = subtle rounding)
        // cursor: pointer: Show pointer cursor on hover (indicates clickable)
        // font-weight: 600: Semi-bold text
        // min-width: 100px: Minimum button width (so small text doesn't make tiny buttons)
        // border-bottom: 3px solid BRIGHT_RED: A bottom border for 3D effect
        // box-shadow: Subtle shadow under the button
        // outline: 2px solid border around the button
        // transition: all 0.3s ease: Smooth animation for hover effects (see CSS)
        "padding: 12px 24px; margin: 5px; background: linear-gradient(90deg, {} 0%, {} 100%); \
         color: #fff; border: none; border-radius: 6px; cursor: pointer; \
         font-weight: 600; min-width: 100px; border-bottom: 3px solid {}; \
         box-shadow: 0 2px 8px #1a0000; outline: 2px solid #2d232b; transition: all 0.3s ease;",
        theme::EVIL_RED,
        theme::DARK_GREY,
        theme::BRIGHT_RED
    );

    // Render a button element with the style and click handler
    view! {
        // The `on:click` attribute binds the on_click function to click events
        // Leptos automatically passes the MouseEvent to the handler
        <button on:click=on_click style=button_style>
            // Render the label text inside the button
            {label}
        </button>
    }
}

// ============================================================================
// SECTION 4: FEATURE COMPONENTS (Counter Logic)
// ============================================================================
//
// WHAT: These components implement the specific features of your app (the counter).
//
// WHY: Separating features from layout makes it easier to understand how
//      the counter works in isolation.

/// CounterDisplay - Shows the current count value
///
/// WHAT IT DOES:
/// - Displays "Count: " followed by the current count in large text
/// - Takes a closure that returns the current count value
/// - When the count changes, this component automatically re-renders
///
/// PROP EXPLANATION:
/// - `count`: A closure (function) that returns i32 (an integer)
///   - `impl Fn() -> i32` means "any type of function that takes no arguments and returns i32"
///   - `Send + Sync + 'static` means the function can be safely shared across threads
///
/// WHY A CLOSURE:
/// We pass a CLOSURE (function) instead of a value because we need the CURRENT value
/// every time the component renders. If we passed `count.get()` (a value), it would
/// be "frozen" - it wouldn't update when count changes.
///
/// LEARNING: Use closures for reactive values. Use plain values for static content.
///
/// REACTIVITY IN ACTION:
/// When the parent component's `count` signal changes:
/// 1. Leptos detects the change
/// 2. It calls `count()` (our closure) again
/// 3. Gets the new value
/// 4. Re-renders this component with the new display
/// This happens automatically - we don't need to write update code.
#[component]
fn CounterDisplay(
    /// A closure that returns the current count value
    count: impl Fn() -> i32 + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        // A paragraph with muted color
        <p style=format!(
            "color: {}; font-size: 1.2em; margin-bottom: 24px;",
            theme::TEXT_MUTED
        )>
            // Static text
            "Count: "

            // A span (inline element) that displays the count in large red text
            <span style=format!(
                "font-size: 2.5em; font-weight: bold; color: {};",
                theme::BRIGHT_RED
            )>
                // Call the closure to get the current count
                // Wrapping it in `move || count()` creates a closure that Leptos can track
                {move || count()}
            </span>
        </p>
    }
}

/// CounterMessage - Shows different messages based on the count value
///
/// WHAT IT DOES:
/// - Displays a message that changes based on how high/low the count is
/// - Examples:
///   - "🔥 Count is HIGH! 75 is above 50!" when count > 50
///   - "❄️ Count is NEGATIVE! -5 is below zero!" when count < 0
///   - "😴 Count is ZERO! Reset complete!" when count == 0
///   - "Count is normal (1-49)" otherwise
///
/// KEY LEARNING: CONDITIONAL RENDERING WITH REACTIVITY
/// ============================================================
///
/// THE PROBLEM:
/// In many UI frameworks, you can do this:
/// ```rust
/// view! {
///     if count > 50 {
///         <p>"Count is high"</p>
///     } else {
///         <p>"Count is normal"</p>
///     }
/// }
/// ```
///
/// BUT IN LEPTOS, this doesn't work the way you'd expect!
/// Why? Because each branch (the if and else) returns a DIFFERENT TYPE.
/// One returns DomFragment<HighMessage>, the other returns DomFragment<NormalMessage>.
/// Rust can't mix different types in one expression, so this causes a compiler error.
///
/// THE SOLUTION: Compute the VALUES (not the views), then render ONE view structure.
/// ```rust
/// view! {
///     {
///         // Step 1: Use Rust if/else to compute the message and color
///         let (message, color) = if count > 50 {
///             ("High", RED)
///         } else {
///             ("Normal", GREY)
///         };
///
///         // Step 2: Render ONE view with DYNAMIC content
///         move || {
///             view! {
///                 <p style=format!("color: {}", color)>
///                     {message}
///                 </p>
///             }
///         }
///     }
/// }
/// ```
///
/// WHY THIS WORKS:
/// - The Rust if/else computes the values BEFORE rendering
/// - The view! macro only sees ONE view structure (<p>)
/// - The closure wrapping lets Leptos track when things change
/// - No type conflicts!
///
/// PROP EXPLANATION:
/// - `count: ReadSignal<i32>` - This is a signal (reactive value)
///   - NOT a closure, but the actual signal itself
///   - Leptos will track when this signal changes
///   - We call `.get()` on it to extract the current value
///
/// THE CRUCIAL PATTERN:
/// When you have a signal (ReadSignal), you MUST access it inside a closure
/// so Leptos can track the dependency. This is why we have `move || { ... }`.
/// Without the closure, Leptos doesn't know "this component depends on that signal".
#[component]
fn CounterMessage(count: ReadSignal<i32>) -> impl IntoView {
    view! {
        {
            // The outer braces {} in the view! macro let us write Rust code
            // (not just HTML). This is how we embed closures, variables, etc.

            // Wrap the entire computation in a closure so Leptos can track it
            // Key: This closure will RE-RUN every time `count` changes
            // The `move` keyword means the closure captures `count` by moving it
            move || {
                // Get the current count value
                // This is WHERE Leptos tracks the dependency:
                // "This closure depends on the `count` signal"
                let current = count.get();

                // Use Rust if/else to COMPUTE the message text and color
                // We're building Rust values here, NOT views yet
                let (message, color) = if current > 50 {
                    // HIGH: return a tuple of (message_text, color)
                    (
                        "🔥 Count is HIGH! ".to_string() + &current.to_string() + " is above 50!",
                        theme::BRIGHT_RED,
                    )
                } else if current < 0 {
                    // NEGATIVE: return a tuple of (message_text, color)
                    (
                        "❄️ Count is NEGATIVE! ".to_string() + &current.to_string() + " is below zero!",
                        "#6bb6ff",
                    )
                } else if current == 0 {
                    // ZERO: return a tuple of (message_text, color)
                    (
                        "😴 Count is ZERO! Reset complete!".to_string(),
                        theme::TEXT_MUTED,
                    )
                } else {
                    // NORMAL: return a tuple of (message_text, color)
                    ("Count is normal (1-49)".to_string(), theme::TEXT_MUTED)
                };

                // Build the CSS style with the computed color
                let style = format!(
                    "color: {}; font-size: 1.1em; margin-top: 12px; font-weight: bold;",
                    color
                );

                // NOW render the view with the computed values
                // This is a SINGLE view structure, so no type conflicts
                view! {
                    <p style=style>
                        // The message text will automatically update when count changes
                        // because this entire closure re-runs when count changes
                        {message}
                    </p>
                }
            }
        }
    }
}

/// CounterButtons - Renders all the action buttons for the counter
///
/// WHAT IT DOES:
/// - Displays 5 buttons: -1, +1, Reset, *2, /2
/// - Each button has a click handler passed as a prop
/// - When clicked, the handler updates the parent's count signal
///
/// PROPS EXPLANATION:
/// - `on_decrement`, `on_increment`, `on_reset`, `on_multiply`, `on_divide`
///   - All are Box<dyn Fn(leptos::ev::MouseEvent) + 'static>
///   - This means "a boxed function that takes a MouseEvent and returns nothing"
///   - We receive DIFFERENT functions for each button (decrement for -1, increment for +1, etc.)
///
/// LEARNING: By accepting functions as props, we make this component completely
///           generic. It doesn't care WHAT the buttons do - it just calls the functions.
///           The parent component decides what each button does.
///
/// COMPOSITION IN ACTION:
/// This component uses our `EvilButton` component 5 times.
/// This shows how components can be composed (used inside other components).
#[component]
fn CounterButtons(
    /// Callback when "-1" button is clicked
    on_decrement: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,

    /// Callback when "+1" button is clicked
    on_increment: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,

    /// Callback when "Reset" button is clicked
    on_reset: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,

    /// Callback when "*2" button is clicked
    on_multiply: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,

    /// Callback when "/2" button is clicked
    on_divide: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
) -> impl IntoView {
    view! {
        // Container for buttons with some spacing
        <div style="margin: 20px 0;">
            // Each EvilButton uses our reusable button component
            // We pass:
            // 1. label - what text to show on the button
            // 2. on_click - which handler to call when clicked
            <EvilButton label="-1" on_click=on_decrement />
            <EvilButton label="+1" on_click=on_increment />
            <EvilButton label="Reset" on_click=on_reset />
            <EvilButton label="*2" on_click=on_multiply />
            <EvilButton label="/2" on_click=on_divide />
        </div>
    }
}

/// Counter - The main feature component that manages count state
///
/// WHAT IT DOES:
/// - Manages the count state (using a Leptos signal)
/// - Creates event handlers that update the count
/// - Composes CounterDisplay, CounterMessage, and CounterButtons into a UI
///
/// KEY CONCEPTS: SIGNALS & STATE MANAGEMENT
/// ===========================================
///
/// WHAT ARE SIGNALS?
/// Signals are Leptos's way of managing reactive state. When a signal changes,
/// Leptos automatically updates all components that depend on it.
/// This is the CORE of Leptos's reactivity system.
///
/// CREATING A SIGNAL:
/// ```rust
/// let (count, set_count) = signal(0);
/// ```
/// This creates:
/// - `count`: A ReadSignal<i32> - you can READ the current value
/// - `set_count`: A WriteSignal<i32> - you can WRITE/UPDATE the value
/// - Initial value: 0
///
/// The naming convention is (getter, setter) or (value, set_value).
///
/// READING A SIGNAL:
/// ```rust
/// let current = count.get();  // Gets the current value
/// ```
///
/// UPDATING A SIGNAL:
/// There are two ways:
/// ```rust
/// set_count.set(42);           // Replace with 42
/// set_count.update(|c| *c += 1); // Modify the current value
/// ```
///
/// EVENT HANDLERS & CLOSURES:
/// ===========================
///
/// What's a closure? A function you define inline:
/// ```rust
/// let add_one = |x| x + 1;     // A closure
/// add_one(5);                   // Returns 6
/// ```
///
/// The `move` keyword in closures means "capture variables by MOVING them".
/// Without `move`, the closure tries to BORROW, which can cause lifetime issues.
/// In event handlers, we always use `move` because the handler might outlive
/// the function that created it.
///
/// EVENT HANDLER PATTERN:
/// ```rust
/// let handle_increment = move |_: leptos::ev::MouseEvent| {
///     // The `_` means "we don't care about the event details"
///     // We just want to know the button was clicked
///     set_count.update(|c| *c += 1);
/// };
/// ```
///
/// COMPOSING COMPONENTS:
/// =======================
///
/// After we create our state and handlers, we compose them into a view:
/// 1. CounterDisplay - shows the current count
/// 2. CounterMessage - shows a message based on the count
/// 3. CounterButtons - renders the buttons with handlers
///
/// Each child component receives what it needs as props:
/// - CounterDisplay gets a closure that returns the count
/// - CounterMessage gets the count signal directly
/// - CounterButtons gets the event handlers
///
/// This separation of concerns makes each component easy to understand and test.
#[component]
fn Counter() -> impl IntoView {
    // ========================================================================
    // STATE MANAGEMENT
    // ========================================================================
    // Create a reactive signal for the count
    // - `count`: Read the current value with count.get()
    // - `set_count`: Update the value with set_count.set() or set_count.update()
    // - Initial value: 0
    let (count, set_count) = signal(0);

    // ========================================================================
    // EVENT HANDLERS
    // ========================================================================
    // Each handler is a closure that captures `set_count` and updates the count
    //
    // The |_: leptos::ev::MouseEvent| syntax means:
    // - | | - start of closure
    // - _ - we're not using the MouseEvent parameter (the _ means "ignore this")
    // - : leptos::ev::MouseEvent - the parameter TYPE
    // - | - end of closure parameters

    // Decrement: subtract 1 from count
    let handle_decrement = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c -= 1);

    // Increment: add 1 to count
    let handle_increment = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c += 1);

    // Reset: set count to 0
    let handle_reset = move |_: leptos::ev::MouseEvent| set_count.set(0);

    // Multiply: double the count (multiply by 2)
    let handle_multiply = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c *= 2);

    // Divide: halve the count (integer division by 2)
    let handle_divide = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c /= 2);

    // ========================================================================
    // STYLING
    // ========================================================================
    // Build the CSS for the container div
    // This groups all the counter UI together in a styled card
    let container_style = format!(
        // padding: Space inside the card
        // text-align: center: Center-align text
        // background: Card background color (from theme)
        // border-radius: Rounded corners
        // max-width: Don't let it get too wide on large screens
        // border: A red border to define the edge
        // box-shadow: A subtle shadow for depth
        "padding: 32px; text-align: center; background: {}; border-radius: 12px; \
         max-width: 600px; border: 2px solid {}; box-shadow: 0 10px 40px rgba(139,0,0,0.4);",
        theme::CARD_BG,
        theme::EVIL_RED
    );

    // ========================================================================
    // RENDERING
    // ========================================================================
    view! {
        // The main container for the counter UI
        <div style=container_style>
            // PART 1: Display the current count
            // Pass a closure so it updates reactively when count changes
            <CounterDisplay count=move || count.get() />

            // PART 2: Show a message based on the count value
            // Pass the signal directly (not a closure)
            // CounterMessage will wrap it in a closure itself
            <CounterMessage count=count />

            // PART 3: Render the action buttons
            // Pass each handler as a Box (pointer) to the function
            // When a button is clicked, its handler is called, which updates set_count
            <CounterButtons
                on_decrement=Box::new(handle_decrement)
                on_increment=Box::new(handle_increment)
                on_reset=Box::new(handle_reset)
                on_multiply=Box::new(handle_multiply)
                on_divide=Box::new(handle_divide)
            />
        </div>
    }
}

// ============================================================================
// SECTION 5: APPLICATION STRUCTURE
// ============================================================================
//
// WHAT: The top-level app structure that ties everything together.
//
// WHY: It shows how a complete Leptos app is organized:
//      1. A root App component
//      2. That combines layout and feature components
//      3. That gets mounted to the browser DOM

/// App - The root component of the entire application
///
/// WHAT IT DOES:
/// - Combines the layout (EvilBackground) with the feature (Counter)
/// - This is the entry point for the entire UI
/// - Everything else is nested inside this component
///
/// COMPONENT HIERARCHY:
/// ```
/// App
/// ├── EvilBackground (layout)
/// │   └── Counter (feature)
/// │       ├── CounterDisplay
/// │       ├── CounterMessage
/// │       └── CounterButtons
/// │           ├── EvilButton
/// │           ├── EvilButton
/// │           ├── EvilButton
/// │           ├── EvilButton
/// │           └── EvilButton
/// ```
///
/// LEARNING: This tree structure is how you build complex UIs.
///           Start with simple components at the bottom (EvilButton),
///           compose them into feature components (CounterButtons),
///           then combine features into pages (Counter),
///           then combine pages into the app (App).
#[component]
fn App() -> impl IntoView {
    view! {
        // The entire app is wrapped in the background layout
        <EvilBackground>
            // Inside the background, we render the counter feature
            <Counter />
        </EvilBackground>
    }
}

// ============================================================================
// SECTION 6: ENTRY POINT - BOOTSTRAPPING THE APP
// ============================================================================
//
// WHAT: The code that starts the entire app and mounts it to the browser.
//
// WHY: WebAssembly needs an entry point to know where to start execution.
//      The #[wasm_bindgen(start)] attribute tells wasm-bindgen
//      "This function should be called when the WASM module loads".

/// main - The entry point that mounts the Leptos app to the DOM
///
/// WHAT IT DOES:
/// 1. Imports the mount_to_body function from Leptos
/// 2. Calls mount_to_body with a closure that returns the App component
/// 3. Leptos renders the App to the <body> element in index.html
///
/// HOW IT WORKS:
/// - The #[wasm_bindgen(start)] attribute marks this as the WASM entry point
/// - When the browser loads the WASM module, it calls this function
/// - pub fn main() is called, which mounts the app
/// - Leptos takes over, rendering the app and managing updates
///
/// THE mount_to_body FUNCTION:
/// ```rust
/// mount_to_body(|| view! { <App /> })
/// ```
/// - Takes a closure that returns a view
/// - The closure is evaluated ONCE to render the app
/// - The closure is also kept to re-render when signals change
/// - mount_to_body renders this view as the direct children of <body>
///
/// INDEX.HTML CONNECTION:
/// The index.html file (in the project root) looks like:
/// ```html
/// <body>
///     <!-- This is where mount_to_body renders the app -->
/// </body>
/// ```
/// After our Rust code runs, <body> contains the entire Leptos app!
///
/// LEARNING: This is how you connect Rust code to the browser.
///           The Rust compiles to WASM, the WASM runs in the browser,
///           and Leptos renders your components to actual DOM elements.
/// - pub fn main() is called, which mounts the app
/// - Leptos takes over, rendering the app and managing updates
#[wasm_bindgen(start)]
pub fn main() {
    use leptos::mount::mount_to_body;

    // Mount the App component to the <body> element
    // This renders the entire application and starts the reactivity system
    mount_to_body(|| view! { <App /> });
}

// ============================================================================
// COMPREHENSIVE LEARNING GUIDE: UNDERSTANDING LEPTOS REACTIVITY
// ============================================================================
//
// If you want to understand how Leptos works deeply, read this section.
// It explains the philosophy behind Leptos and why it works the way it does.

// --- CONCEPT 1: THE VIEW! MACRO ---
// The view! macro is Leptos's special syntax for writing HTML.
// It looks like HTML but it's actually Rust code.
//
// These TWO are equivalent:
//
// Using view! macro:
//   view! { <p>"Hello"</p> }
//
// Raw Leptos API (you rarely use this):
//   HtmlElement::p([Text::new("Hello")])
//
// The macro is just syntactic sugar. It gets expanded at compile time.
// This is why you get compile errors if your HTML-like syntax is invalid -
// the macro can't parse it properly.
//
// HOW IT WORKS:
// 1. You write: view! { <button on:click=handler>"Click me"</button> }
// 2. Rust compiler sees view! and calls the leptos_macro procedural macro
// 3. The macro parses your HTML-like syntax
// 4. It generates Rust code that creates DOM elements
// 5. That code gets compiled to WASM
// 6. At runtime, the WASM creates actual DOM elements

// --- CONCEPT 2: SIGNALS & REACTIVITY ---
// Signals are the CORE of Leptos's reactivity system.
//
// WITHOUT SIGNALS (bad):
//   let mut count = 0;
//   count = 5;  // You change the variable
//   // But the UI doesn't know it changed - it's still showing 0
//
// WITH SIGNALS (good):
//   let (count, set_count) = signal(0);
//   set_count.set(5);  // You call a setter
//   // Leptos detects the change
//   // All components that USE count automatically re-render with the new value
//
// This is called REACTIVE because the UI REACTS to state changes automatically.
//
// HOW LEPTOS TRACKS DEPENDENCIES:
// When you call count.get() inside a closure that's used in view! markup,
// Leptos records: "This closure depends on the count signal".
// When count changes, Leptos re-runs that closure and updates the DOM.
//
// IMPORTANT: Leptos only tracks .get() calls INSIDE the view! macro.
// If you call .get() outside the view!, you get a value, not a dependency.
// That's why CounterMessage has move || wrapped around everything -
// to make sure the dependency tracking happens.

// --- CONCEPT 3: COMPONENTS AS FUNCTIONS ---
// In Leptos, components are just Rust functions with the #[component] attribute.
//
// A component:
// 1. Takes props as function parameters
// 2. Returns impl IntoView (something that can be rendered to HTML)
// 3. Can create state (signals)
// 4. Can render other components
//
// EXAMPLE:
//   #[component]
//   fn MyButton(label: String) -> impl IntoView {
//       view! { <button>{label}</button> }
//   }
//
// You use it like a React component:
//   view! { <MyButton label="Click me" /> }
//
// The #[component] macro handles all the magic:
// - Converting HTML-like prop syntax to function arguments
// - Creating the component tree
// - Wiring up reactivity

// --- CONCEPT 4: CLOSURES IN EVENT HANDLERS ---
// Event handlers are just closures that take an event and do something.
//
// PATTERN:
//   let handler = move |event: MouseEvent| {
//       // Do something with the event
//   };
//   view! { <button on:click=handler>Click</button> }
//
// WHY move:
// The handler needs to capture variables (like set_count).
// `move` means capture variables by moving them (transferring ownership).
// Without `move`, Rust complains about lifetimes.
// In practice, always use `move` for event handlers.
//
// THE EVENT OBJECT:
// event contains details about what happened:
//   - event.client_x, event.client_y - mouse position
//   - event.key - keyboard key pressed
//   - etc.
//
// Often we don't care about the event details (just that it happened),
// so we use the `_` pattern: |_: MouseEvent| means "ignore the event".

// --- CONCEPT 5: COMPONENT COMPOSITION ---
// The REAL power of components is that you can compose them.
//
// EXAMPLE:
// 1. Build small, simple components (EvilButton, CounterDisplay)
// 2. Compose them into feature components (Counter)
// 3. Compose features into pages (App)
// 4. Compose pages into the whole app
//
// Each layer is easier to understand because each component does ONE thing.
//
// BENEFITS:
// - Easy to test (test each component in isolation)
// - Easy to understand (small, focused components)
// - Easy to reuse (if you need the same feature twice, just use the component twice)
// - Easy to modify (change one component, everything that uses it is updated)

// --- CONCEPT 6: PROPS (COMPONENT PARAMETERS) ---
// Props let you configure components from the outside.
//
// EXAMPLE:
//   #[component]
//   fn Counter(initial_count: i32, step: i32) -> impl IntoView {
//       let (count, set_count) = signal(initial_count);
//       // ... increment by step instead of 1
//   }
//
// Usage:
//   view! { <Counter initial_count=10 step=5 /> }
//
// Different props = different behavior, same component = code reuse.

// --- CONCEPT 7: STYLED COMPONENTS ---
// Notice how every component has a `style=` attribute with CSS?
//
// That's one way to do styling in Leptos:
// - Inline styles (what we did)
// - CSS classes (link to index.html CSS)
// - Tailwind classes
// - CSS-in-JS libraries
//
// Inline styles are simpler but less scalable.
// For a larger app, you'd use CSS classes or a framework like Tailwind.
//
// Our approach (inline styles with format!) is good for:
// - Learning (you see all the styling in one place)
// - Small apps (not too much duplicated code)
// - Dynamic styling based on state

// --- CONCEPT 8: LEARNING PROGRESSION ---
// If you want to extend this app, here's a good progression:
//
// 1. Add a second counter (requires: new signal in App, new Counter instance)
// 2. Add localStorage (requires: understanding browser APIs, .set_untracked())
// 3. Add an input field to set count manually (requires: two-way binding)
// 4. Add a history of operations (requires: Vec<Signal<T>> or similar)
// 5. Extract CSS to separate file (requires: understanding CSS)
// 6. Add themes switching (requires: passing signals as props)
//
// Each step teaches you more about Leptos and web development.

// ============================================================================
// YOU'VE REACHED THE END!
// ============================================================================
//
// Congratulations! You've now seen:
// ✓ Component basics (#[component] attribute)
// ✓ Props (configuring components)
// ✓ Signals (reactive state)
// ✓ Event handlers (closures in HTML)
// ✓ Reactivity (automatic UI updates)
// ✓ Component composition (building from simple to complex)
// ✓ Styling (inline CSS with Rust)
// ✓ The view! macro (HTML-like syntax)
// ✓ Entry points and mounting (#[wasm_bindgen(start)])
//
// NEXT STEPS:
// 1. Run the app and play with it: trunk serve --open
// 2. Make small changes to the code and see what happens
// 3. Try one of the suggested extensions above
// 4. Read the Leptos official docs: https://leptos.dev
// 5. Build your own project using these patterns
//
// Remember: The best way to learn is by DOING. Change things, break things,
// fix them, and learn from the process. You've got this!
