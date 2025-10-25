use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn Counter() -> impl IntoView {
    // Create a signal for the counter state
    // Signals are the foundation of Leptos reactivity
    let (count, set_count) = signal(0);

    // Event handlers using closures
    let increment = move |_| set_count.update(|c| *c += 1);
    let decrement = move |_| set_count.update(|c| *c -= 1);
    let reset = move |_| set_count.set(0);

    view! {
        <div class="counter-container">
            <h1>"Welcome to Leptos!"</h1>
            <p>"This is a simple counter to get you started with Rust and HTML."</p>

            <div class="counter-display">
                <p>"Current count: " <span class="count-value">{count}</span></p>
            </div>

            <div class="button-group">
                <button on:click=decrement class="btn btn-decrease">
                    "-1"
                </button>
                <button on:click=increment class="btn btn-increase">
                    "+1"
                </button>
                <button on:click=reset class="btn btn-reset">
                    "Reset"
                </button>
            </div>

            <div class="info">
                <h2>"What's happening here?"</h2>
                <ul>
                    <li>"The " <code>"signal(0)"</code> " creates reactive state"</li>
                    <li>"When you click a button, the counter updates"</li>
                    <li>"Leptos automatically re-renders only what changed"</li>
                    <li>"All of this is compiled from Rust to WebAssembly!"</li>
                </ul>
            </div>
        </div>
    }
}

/// Main App component
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <Counter />
        </div>
    }
}

/// Entry point - mounts the app to the DOM
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    mount_to_body(|| view! { <App /> })
}
