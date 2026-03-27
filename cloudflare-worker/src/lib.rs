#[allow(dead_code)]
#[path = "../../backend/src/calculators/financial.rs"]
mod financial;
#[path = "../../backend/src/calculators/health.rs"]
mod health;
#[path = "../../backend/src/calculators/math.rs"]
mod math;
#[path = "../../backend/src/calculators/technical.rs"]
mod technical;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use worker::*;

#[derive(Serialize)]
struct HealthPayload {
    status: &'static str,
    service: &'static str,
    mode: &'static str,
}

#[derive(Debug, Deserialize)]
struct PercentageInput {
    value: f64,
    percentage: f64,
    calculation_type: PercentageType,
}

#[derive(Debug, Deserialize)]
enum PercentageType {
    WhatIsXPercentOfY,
    XIsWhatPercentOfY,
    PercentageChange,
}

#[derive(Debug, Serialize)]
struct PercentageResult {
    result: f64,
    formula: String,
}

#[derive(Debug, Deserialize)]
struct ExchangeRateApiResponse {
    rates: HashMap<String, f64>,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let path = req.path();
    let method = req.method().clone();
    let allowed_origin = configured_origin(&env);

    let mut response = if method == Method::Options && path.starts_with("/api/") {
        cors_preflight()?
    } else {
        match (method, path.as_str()) {
            (Method::Get, "/api/hello") => {
                Response::ok("Hello from CalculatorD on Cloudflare Workers")?
            }
            (Method::Get, "/api/health") => Response::from_json(&HealthPayload {
                status: "ok",
                service: "CalculatorD",
                mode: "cloudflare-native-worker",
            })?,
            (Method::Post, "/api/mortgage") => {
                json_route::<financial::MortgageInput, _, _>(req, financial::calculate_mortgage)
                    .await?
            }
            (Method::Post, "/api/compound-interest") => {
                json_route::<financial::CompoundInterestInput, _, _>(
                    req,
                    financial::calculate_compound_interest,
                )
                .await?
            }
            (Method::Post, "/api/auto-loan") => {
                json_route::<financial::AutoLoanInput, _, _>(req, financial::calculate_auto_loan)
                    .await?
            }
            (Method::Post, "/api/inflation") => {
                json_route::<financial::InflationInput, _, _>(req, financial::calculate_inflation)
                    .await?
            }
            (Method::Post, "/api/currency") => {
                json_route_async::<financial::CurrencyInput, _, _, _>(req, convert_currency_worker)
                    .await?
            }
            (Method::Post, "/api/investment") => {
                json_route::<financial::InvestmentInput, _, _>(req, financial::calculate_investment)
                    .await?
            }
            (Method::Post, "/api/bmi") => {
                json_route::<health::BMIInput, _, _>(req, health::calculate_bmi).await?
            }
            (Method::Post, "/api/tdee") => {
                json_route::<health::TDEEInput, _, _>(req, health::calculate_tdee).await?
            }
            (Method::Post, "/api/body-metrics") => {
                json_route::<health::BodyMetricsInput, _, _>(req, health::calculate_body_metrics)
                    .await?
            }
            (Method::Post, "/api/pregnancy") => {
                json_route::<health::PregnancyInput, _, _>(req, health::calculate_pregnancy).await?
            }
            (Method::Post, "/api/quick-math") => {
                json_route::<math::QuickMathInput, _, _>(req, math::calculate_quick_math).await?
            }
            (Method::Post, "/api/unit-convert") => {
                json_route::<math::UnitConversionInput, _, _>(req, math::convert_units).await?
            }
            (Method::Post, "/api/graph") => {
                json_route::<math::GraphInput, _, _>(req, math::graph_quadratic).await?
            }
            (Method::Post, "/api/gpa") => {
                json_route::<math::GpaInput, _, _>(req, math::calculate_gpa).await?
            }
            (Method::Post, "/api/fraction") => {
                json_route::<math::FractionInput, _, _>(req, math::solve_fraction).await?
            }
            (Method::Post, "/api/scientific") => {
                json_route::<math::ScientificInput, _, _>(req, math::calculate_scientific).await?
            }
            (Method::Post, "/api/equation") => {
                json_route::<math::EquationInput, _, _>(req, math::solve_equation).await?
            }
            (Method::Post, "/api/percentage") => {
                json_route::<PercentageInput, _, _>(req, calculate_percentage).await?
            }
            (Method::Post, "/api/base64/encode") => {
                json_route::<technical::Base64Input, _, _>(req, technical::encode_base64).await?
            }
            (Method::Post, "/api/base64/decode") => {
                json_route::<technical::Base64Input, _, _>(req, technical::decode_base64).await?
            }
            (Method::Post, "/api/json/format") => {
                json_route::<technical::JsonFormatInput, _, _>(req, technical::format_json).await?
            }
            (Method::Post, "/api/epoch") => {
                json_route::<technical::EpochInput, _, _>(req, technical::convert_epoch).await?
            }
            (Method::Post, "/api/business-days") => {
                json_route::<technical::BusinessDaysInput, _, _>(
                    req,
                    technical::calculate_business_days,
                )
                .await?
            }
            (Method::Post, "/api/age") => {
                json_route::<technical::AgeInput, _, _>(req, technical::calculate_age).await?
            }
            (Method::Post, "/api/password") => {
                json_route::<technical::PasswordInput, _, _>(req, technical::generate_password)
                    .await?
            }
            (Method::Post, "/api/timezone") => {
                json_route::<technical::TimeZoneInput, _, _>(req, technical::convert_time_zone)
                    .await?
            }
            (Method::Post, "/api/concrete") => {
                json_route::<technical::ConcreteInput, _, _>(req, technical::estimate_concrete)
                    .await?
            }
            (Method::Post, "/api/tiles") => {
                json_route::<technical::TileInput, _, _>(req, technical::estimate_tiles).await?
            }
            _ => Response::error("Not Found", 404)?,
        }
    };

    apply_cors(&mut response, &allowed_origin)?;
    Ok(response)
}

async fn json_route<T, U, F>(mut req: Request, handler: F) -> Result<Response>
where
    T: DeserializeOwned,
    U: Serialize,
    F: FnOnce(T) -> U,
{
    let payload = req.json::<T>().await?;
    Response::from_json(&handler(payload))
}

async fn json_route_async<T, U, F, Fut>(mut req: Request, handler: F) -> Result<Response>
where
    T: DeserializeOwned,
    U: Serialize,
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = Result<U>>,
{
    let payload = req.json::<T>().await?;
    Response::from_json(&handler(payload).await?)
}

async fn convert_currency_worker(
    input: financial::CurrencyInput,
) -> Result<financial::CurrencyResult> {
    let from = input.from.to_uppercase();
    let to = input.to.to_uppercase();
    let url = format!("https://open.er-api.com/v6/latest/{from}");

    let request = Request::new(&url, Method::Get)?;
    if let Ok(mut response) = Fetch::Request(request).send().await {
        if let Ok(body) = response.json::<ExchangeRateApiResponse>().await {
            if let Some(rate) = body.rates.get(&to) {
                return Ok(financial::CurrencyResult {
                    converted_amount: input.amount * rate,
                    rate: *rate,
                    source: "open.er-api.com".to_string(),
                    fetched_at: chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
                });
            }
        }
    }

    let rate = fallback_rate(&from, &to);
    Ok(financial::CurrencyResult {
        converted_amount: input.amount * rate,
        rate,
        source: "fallback".to_string(),
        fetched_at: chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
    })
}

fn fallback_rate(from: &str, to: &str) -> f64 {
    if from == to {
        return 1.0;
    }

    let usd_rates = HashMap::from([
        ("USD", 1.0),
        ("EUR", 0.92),
        ("GBP", 0.79),
        ("NPR", 133.0),
        ("INR", 83.0),
        ("JPY", 151.0),
        ("CAD", 1.35),
        ("AUD", 1.52),
        ("CHF", 0.88),
        ("CNY", 7.18),
        ("AED", 3.67),
        ("SGD", 1.35),
    ]);

    match (usd_rates.get(from), usd_rates.get(to)) {
        (Some(from_rate), Some(to_rate)) if *from_rate > 0.0 => to_rate / from_rate,
        _ => 1.0,
    }
}

fn calculate_percentage(input: PercentageInput) -> PercentageResult {
    match input.calculation_type {
        PercentageType::WhatIsXPercentOfY => {
            let result = (input.percentage / 100.0) * input.value;
            PercentageResult {
                result,
                formula: format!("{}% of {} = {}", input.percentage, input.value, result),
            }
        }
        PercentageType::XIsWhatPercentOfY => {
            let result = (input.value / input.percentage.max(1.0)) * 100.0;
            PercentageResult {
                result,
                formula: format!("{} is {}% of {}", input.value, result, input.percentage),
            }
        }
        PercentageType::PercentageChange => {
            let result =
                ((input.value - input.percentage) / input.percentage.abs().max(1.0)) * 100.0;
            PercentageResult {
                result,
                formula: format!(
                    "Change from {} to {} = {}%",
                    input.percentage, input.value, result
                ),
            }
        }
    }
}

fn configured_origin(env: &Env) -> String {
    env.var("APP_ORIGIN")
        .map(|value| value.to_string())
        .unwrap_or_else(|_| "https://calculatord.calculatorq.workers.dev".to_string())
}

fn cors_preflight() -> Result<Response> {
    Ok(Response::empty()?.with_status(204))
}

fn apply_cors(response: &mut Response, allowed_origin: &str) -> Result<()> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", allowed_origin)?;
    headers.set(
        "Access-Control-Allow-Methods",
        "GET, POST, PUT, PATCH, DELETE, OPTIONS",
    )?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization, X-Requested-With",
    )?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    headers.set("Vary", "Origin")?;
    Ok(())
}
