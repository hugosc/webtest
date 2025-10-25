use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
// A component is a reusable piece of UI that includes logic. This one is a counter.
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(32); // A signal is a reactive state variable. It holds the current count value.

    let increment = move |_| set_count.update(|c| *c += 1); // move |_| creates a closure that captures set_count by value.
    let decrement = move |_| set_count.update(|c| *c -= 1);
    let reset = move |_| set_count.set(0);
    let multiply_by_two = move |_| set_count.update(|c| *c *= 2);
    // Divide the current count by two (integer division)
    let divide_by_two = move |_| set_count.update(|c| *c /= 2);

    view! {
        <div style="display: flex; justify-content: center; align-items: center; min-height: 100vh;">
            <div style="padding: 20px; text-align: center; background: white; border-radius: 8px; max-width: 600px;">
                <h1>"Counter"</h1>
                <p>"Count: " <span style="font-size: 2em; font-weight: bold; color: #667eea;">{count}</span></p>
                <div style="margin: 20px 0;">
                <button on:click=decrement style="padding: 10px 20px; margin: 5px; background: #7f0000; color: #222; border: none; border-radius: 4px; cursor: pointer;">"-1"</button>
                <button on:click=increment style="padding: 10px 20px; margin: 5px; background: #222; color: #7f0000; border: none; border-radius: 4px; cursor: pointer;">"+1"</button>
                <button on:click=reset style="padding: 10px 20px; margin: 5px; background: #2d0036; color: #7f0000; border: none; border-radius: 4px; cursor: pointer;">"Reset"</button>
                <button on:click=multiply_by_two style="padding: 10px 20px; margin: 5px; background: #3b003b; color: #7f0000; border: none; border-radius: 4px; cursor: pointer;">"×2"</button>
                <button on:click=divide_by_two style="padding: 10px 20px; margin: 5px; background: #000; color: #7f0000; border: none; border-radius: 4px; cursor: pointer;">"÷2"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
// Even the 'main app' is a component. But in this component we simply call the other components like counter.
fn App() -> impl IntoView {
    view! {
        <Counter />
    }
}

#[wasm_bindgen(start)]
// The entry point for the WebAssembly module. This function is called when the module is loaded.
pub fn main() {
    use leptos::mount::mount_to_body;
    mount_to_body(|| view! { <App /> }); // Mount the App component to the body of the HTML document.
}
