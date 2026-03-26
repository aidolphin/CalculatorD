use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct BmiInput {
    weight: f64,
    height: f64,
    unit_system: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct BmiResult {
    bmi: f64,
    category: String,
    healthy_weight_range: (f64, f64),
    risk_level: String,
}

#[function_component(BmiCalculator)]
pub fn bmi_calculator() -> Html {
    let weight = use_state(|| 70.0);
    let height = use_state(|| 170.0);
    let result = use_state(|| None::<BmiResult>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let calculate = {
        let weight = weight.clone();
        let height = height.clone();
        let result = result.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let weight = *weight;
            let height = *height;
            let result = result.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                let input = BmiInput {
                    weight,
                    height,
                    unit_system: "Metric".to_string(),
                };

                match Request::post("/api/bmi").json(&input) {
                    Ok(request) => match request.send().await {
                        Ok(resp) => match resp.json::<BmiResult>().await {
                            Ok(json) => result.set(Some(json)),
                            Err(_) => error.set(Some("Could not read the BMI result.".to_string())),
                        },
                        Err(_) => error.set(Some("BMI request failed.".to_string())),
                    },
                    Err(_) => error.set(Some("Could not serialize the BMI request.".to_string())),
                }

                loading.set(false);
            });
        })
    };

    html! {
        <section id="health" class="calculator-card">
            <h2>{"BMI Calculator"}</h2>

            <div class="input-group">
                <label for="bmi-weight">{"Weight (kg)"}</label>
                <input
                    id="bmi-weight"
                    type="number"
                    value={weight.to_string()}
                    oninput={let weight = weight.clone(); move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        weight.set(input.value().parse().unwrap_or(0.0));
                    }}
                />
            </div>

            <div class="input-group">
                <label for="bmi-height">{"Height (cm)"}</label>
                <input
                    id="bmi-height"
                    type="number"
                    value={height.to_string()}
                    oninput={let height = height.clone(); move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        height.set(input.value().parse().unwrap_or(0.0));
                    }}
                />
            </div>

            <button onclick={calculate} disabled={*loading}>
                {if *loading { "Calculating..." } else { "Calculate BMI" }}
            </button>

            {
                if let Some(message) = &*error {
                    html! { <p class="result">{message}</p> }
                } else {
                    html! {}
                }
            }

            {if let Some(res) = &*result {
                html! {
                    <div class="result">
                        <h3>{format!("BMI: {:.1}", res.bmi)}</h3>
                        <p>{format!("Category: {}", res.category)}</p>
                        <p>{format!("Healthy Weight Range: {:.1}kg - {:.1}kg", res.healthy_weight_range.0, res.healthy_weight_range.1)}</p>
                        <p>{format!("Risk Level: {}", res.risk_level)}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </section>
    }
}
