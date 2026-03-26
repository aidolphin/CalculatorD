use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;

use components::bmi_calculator::BmiCalculator;
use components::mortgage_calculator::MortgageCalculator;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <nav class="navbar">
                <div class="nav-brand">
                    <h1>{"Calculator.ai"}</h1>
                </div>
                <div class="nav-links">
                    <a href="#financial">{"Financial"}</a>
                    <a href="#health">{"Health"}</a>
                </div>
            </nav>

            <main>
                <MortgageCalculator />
                <BmiCalculator />
            </main>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
