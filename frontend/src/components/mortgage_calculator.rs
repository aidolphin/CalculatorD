use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MortgageInput {
    loan_amount: f64,
    interest_rate: f64,
    loan_term_years: u32,
    property_tax: Option<f64>,
    pmi: Option<f64>,
    down_payment: Option<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct MortgageResult {
    monthly_payment: f64,
    total_payment: f64,
    total_interest: f64,
    payoff_date: String,
}

#[function_component(MortgageCalculator)]
pub fn mortgage_calculator() -> Html {
    let loan_amount = use_state(|| 300_000.0);
    let interest_rate = use_state(|| 4.5);
    let loan_term = use_state(|| 30);
    let result = use_state(|| None::<MortgageResult>);
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    let calculate = {
        let loan_amount = loan_amount.clone();
        let interest_rate = interest_rate.clone();
        let loan_term = loan_term.clone();
        let result = result.clone();
        let error = error.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            let loan_amount = *loan_amount;
            let interest_rate = *interest_rate;
            let loan_term = *loan_term;
            let result = result.clone();
            let error = error.clone();
            let loading = loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                let input = MortgageInput {
                    loan_amount,
                    interest_rate,
                    loan_term_years: loan_term,
                    property_tax: None,
                    pmi: None,
                    down_payment: None,
                };

                match Request::post("/api/mortgage").json(&input) {
                    Ok(request) => match request.send().await {
                        Ok(resp) => match resp.json::<MortgageResult>().await {
                            Ok(json) => result.set(Some(json)),
                            Err(_) => {
                                error.set(Some("Could not read the mortgage result.".to_string()))
                            }
                        },
                        Err(_) => error.set(Some("Mortgage request failed.".to_string())),
                    },
                    Err(_) => error.set(Some(
                        "Could not serialize the mortgage request.".to_string(),
                    )),
                }

                loading.set(false);
            });
        })
    };

    html! {
        <section id="financial" class="calculator-card">
            <h2>{"Mortgage Calculator"}</h2>

            <div class="input-group">
                <label for="loan-amount">{"Loan Amount: $"}</label>
                <input
                    id="loan-amount"
                    type="number"
                    value={loan_amount.to_string()}
                    oninput={let loan_amount = loan_amount.clone(); move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        loan_amount.set(input.value().parse().unwrap_or(0.0));
                    }}
                />
            </div>

            <div class="input-group">
                <label for="interest-rate">{"Interest Rate: %"}</label>
                <input
                    id="interest-rate"
                    type="number"
                    step="0.1"
                    value={interest_rate.to_string()}
                    oninput={let interest_rate = interest_rate.clone(); move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        interest_rate.set(input.value().parse().unwrap_or(0.0));
                    }}
                />
            </div>

            <div class="input-group">
                <label for="loan-term">{"Loan Term: Years"}</label>
                <input
                    id="loan-term"
                    type="number"
                    value={loan_term.to_string()}
                    oninput={let loan_term = loan_term.clone(); move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        loan_term.set(input.value().parse().unwrap_or(30));
                    }}
                />
            </div>

            <button onclick={calculate} disabled={*loading}>
                {if *loading { "Calculating..." } else { "Calculate" }}
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
                        <h3>{format!("Monthly Payment: ${:.2}", res.monthly_payment)}</h3>
                        <p>{format!("Total Payment: ${:.2}", res.total_payment)}</p>
                        <p>{format!("Total Interest: ${:.2}", res.total_interest)}</p>
                        <p>{format!("Payoff Date: {}", res.payoff_date)}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </section>
    }
}
