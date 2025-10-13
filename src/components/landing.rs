use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
        <div class="landing">
            <h1>{ "Welcome to the website!" }</h1>
            <p>{ "This is a simple landing page." }</p>
            <button onclick={Callback::from(|_| log::info!("Button clicked!"))}>
                { "Click Me!" }
            </button>
        </div>
    }
}
