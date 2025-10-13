use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
      <div style="font-family: sans-serif; text-align: center;">
      <h1>{ "helo" }</h1>
      <p>{ "This is a super simple Yew app." }</p>
      </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
