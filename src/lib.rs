use leptos::prelude::*;
use leptos::IntoView;
use wasm_bindgen::prelude::*;

// ============================================================================
// TYPES & CONSTANTS
// ============================================================================

/// Configuration for styling themes used throughout the app
mod theme {
    pub const DARK_GREY: &str = "#232323";
    pub const EVIL_RED: &str = "#8b0000";
    pub const BRIGHT_RED: &str = "#ff1744";
    pub const CARD_BG: &str = "#18141a";
    pub const TEXT_MUTED: &str = "#e57373";
}

// ============================================================================
// COMPONENTS - Layout
// ============================================================================

/// EvilBackground provides the page-wide gradient background and centers content.
/// This is a layout component that wraps the entire app.
#[component]
fn EvilBackground(children: Children) -> impl IntoView {
    let bg_gradient = format!(
        "linear-gradient(135deg, {} 0%, {} 100%)",
        theme::DARK_GREY,
        theme::EVIL_RED
    );

    view! {
        <div style=format!(
            "min-height: 100vh; min-width: 100vw; background: {}; display: flex; justify-content: center; align-items: center; padding: 20px;",
            bg_gradient
        )>
            {children()}
        </div>
    }
}

// ============================================================================
// COMPONENTS - UI Elements
// ============================================================================

/// EvilButton is a reusable button component styled with the evil theme.
/// It takes a label and an event handler as props.
///
/// # Props
/// - `label`: The text to display on the button
/// - `on_click`: Event handler called when button is clicked
#[component]
fn EvilButton(
    #[prop(into)] label: String,
    on_click: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
) -> impl IntoView {
    let button_style = format!(
        "padding: 12px 24px; margin: 5px; background: linear-gradient(90deg, {} 0%, {} 100%); \
         color: #fff; border: none; border-radius: 6px; cursor: pointer; \
         font-weight: 600; min-width: 100px; border-bottom: 3px solid {}; \
         box-shadow: 0 2px 8px #1a0000; outline: 2px solid #2d232b; transition: all 0.3s ease;",
        theme::EVIL_RED,
        theme::DARK_GREY,
        theme::BRIGHT_RED
    );

    view! {
        <button on:click=on_click style=button_style>
            {label}
        </button>
    }
}


// ============================================================================
// COMPONENTS - Feature: Counter
// ============================================================================

/// CounterDisplay shows the current count value.
/// This is a simple presentational component.
#[component]
fn CounterDisplay(count: impl Fn() -> i32 + Send + Sync + 'static) -> impl IntoView {
    view! {
        <p style=format!(
            "color: {}; font-size: 1.2em; margin-bottom: 24px;",
            theme::TEXT_MUTED
        )>
            "Count: "
            <span style=format!(
                "font-size: 2.5em; font-weight: bold; color: {};",
                theme::BRIGHT_RED
            )>
                {move || count()}
            </span>
        </p>
    }
}

/// CounterButtons renders all the action buttons for the counter.
/// This component demonstrates event handling with closures.
#[component]
fn CounterButtons(
    on_decrement: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
    on_increment: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
    on_reset: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
    on_multiply: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
    on_divide: Box<dyn Fn(leptos::ev::MouseEvent) + 'static>,
) -> impl IntoView {
    view! {
        <div style="margin: 20px 0;">
            <EvilButton label="-1" on_click=on_decrement />
            <EvilButton label="+1" on_click=on_increment />
            <EvilButton label="Reset" on_click=on_reset />
            <EvilButton label="*2" on_click=on_multiply />
            <EvilButton label="/2" on_click=on_divide />
        </div>
    }
}

#[component]
fn CounterMessage(count: i32) -> impl IntoView {
    view! {
        <div style="margin: 20px 0;">
            <p>"Count: " {count}</p>
        </div>
    }
}

/// Counter is the main feature component that manages count state and renders UI.
/// It demonstrates:
/// - Using signals for reactive state
/// - Creating closures to update state
/// - Composing smaller components together
#[component]
fn Counter() -> impl IntoView {
    // Reactive state: signal creates a getter and setter for count
    let (count, set_count) = signal(32);

    // Event handlers: each closure captures set_count and updates it
    let handle_decrement = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c -= 1);
    let handle_increment = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c += 1);
    let handle_reset = move |_: leptos::ev::MouseEvent| set_count.set(0);
    let handle_multiply = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c *= 2);
    let handle_divide = move |_: leptos::ev::MouseEvent| set_count.update(|c| *c /= 2);

    // Main layout
    let container_style = format!(
        "padding: 32px; text-align: center; background: {}; border-radius: 12px; \
         max-width: 600px; border: 2px solid {}; box-shadow: 0 10px 40px rgba(139,0,0,0.4);",
        theme::CARD_BG,
        theme::EVIL_RED
    );

    view! {
        <div style=container_style>
            {/* Display the current count */}
            <CounterDisplay count=move || count.get() />

            {/* Render all counter action buttons */}
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
// MAIN APP
// ============================================================================

/// App is the root component that combines all layout and feature components.
/// This demonstrates how to structure a realistic app:
/// 1. Layout components (EvilBackground)
/// 2. Feature components (Counter)
#[component]
fn App() -> impl IntoView {
    view! {
        <EvilBackground>
            <Counter />
        </EvilBackground>
    }
}

// ============================================================================
// ENTRY POINT
// ============================================================================

/// main() is the entry point that mounts the app to the DOM.
/// This is called automatically by the WebAssembly runtime.
#[wasm_bindgen(start)]
pub fn main() {
    use leptos::mount::mount_to_body;
    mount_to_body(|| view! { <App /> });
}
