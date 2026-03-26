use crate::calculators::*;
use rocket::serde::json::Json;

#[post("/mortgage", format = "json", data = "<input>")]
pub fn mortgage_calc(input: Json<financial::MortgageInput>) -> Json<financial::MortgageResult> {
    Json(financial::calculate_mortgage(input.into_inner()))
}

#[post("/compound-interest", format = "json", data = "<input>")]
pub fn compound_interest(
    input: Json<financial::CompoundInterestInput>,
) -> Json<financial::CompoundInterestResult> {
    Json(financial::calculate_compound_interest(input.into_inner()))
}

#[post("/auto-loan", format = "json", data = "<input>")]
pub fn auto_loan(input: Json<financial::AutoLoanInput>) -> Json<financial::AutoLoanResult> {
    Json(financial::calculate_auto_loan(input.into_inner()))
}

#[post("/inflation", format = "json", data = "<input>")]
pub fn inflation_calc(input: Json<financial::InflationInput>) -> Json<financial::InflationResult> {
    Json(financial::calculate_inflation(input.into_inner()))
}

#[post("/bmi", format = "json", data = "<input>")]
pub fn bmi_calc(input: Json<health::BMIInput>) -> Json<health::BMIResult> {
    Json(health::calculate_bmi(input.into_inner()))
}

#[post("/tdee", format = "json", data = "<input>")]
pub fn tdee_calc(input: Json<health::TDEEInput>) -> Json<health::TDEResult> {
    Json(health::calculate_tdee(input.into_inner()))
}

#[post("/body-metrics", format = "json", data = "<input>")]
pub fn body_metrics_calc(input: Json<health::BodyMetricsInput>) -> Json<health::BodyMetricsResult> {
    Json(health::calculate_body_metrics(input.into_inner()))
}

#[post("/pregnancy", format = "json", data = "<input>")]
pub fn pregnancy_calc(input: Json<health::PregnancyInput>) -> Json<health::PregnancyResult> {
    Json(health::calculate_pregnancy(input.into_inner()))
}

#[post("/percentage", format = "json", data = "<input>")]
pub fn percentage_calc(input: Json<PercentageInput>) -> Json<PercentageResult> {
    Json(calculate_percentage(input.into_inner()))
}

#[post("/quick-math", format = "json", data = "<input>")]
pub fn quick_math_calc(input: Json<math::QuickMathInput>) -> Json<math::QuickMathResult> {
    Json(math::calculate_quick_math(input.into_inner()))
}

#[post("/unit-convert", format = "json", data = "<input>")]
pub fn unit_convert(input: Json<math::UnitConversionInput>) -> Json<math::UnitConversionResult> {
    Json(math::convert_units(input.into_inner()))
}

#[post("/graph", format = "json", data = "<input>")]
pub fn graph_calc(input: Json<math::GraphInput>) -> Json<math::GraphResult> {
    Json(math::graph_quadratic(input.into_inner()))
}

#[post("/gpa", format = "json", data = "<input>")]
pub fn gpa_calc(input: Json<math::GpaInput>) -> Json<math::GpaResult> {
    Json(math::calculate_gpa(input.into_inner()))
}

#[post("/fraction", format = "json", data = "<input>")]
pub fn fraction_calc(input: Json<math::FractionInput>) -> Json<math::FractionResult> {
    Json(math::solve_fraction(input.into_inner()))
}

#[post("/scientific", format = "json", data = "<input>")]
pub fn scientific_calc(input: Json<math::ScientificInput>) -> Json<math::ScientificResult> {
    Json(math::calculate_scientific(input.into_inner()))
}

#[post("/equation", format = "json", data = "<input>")]
pub fn equation_calc(input: Json<math::EquationInput>) -> Json<math::EquationResult> {
    Json(math::solve_equation(input.into_inner()))
}

#[post("/currency", format = "json", data = "<input>")]
pub async fn currency_calc(
    input: Json<financial::CurrencyInput>,
) -> Json<financial::CurrencyResult> {
    Json(financial::convert_currency(input.into_inner()).await)
}

#[post("/investment", format = "json", data = "<input>")]
pub fn investment_calc(
    input: Json<financial::InvestmentInput>,
) -> Json<financial::InvestmentResult> {
    Json(financial::calculate_investment(input.into_inner()))
}

#[post("/base64/encode", format = "json", data = "<input>")]
pub fn base64_encode(input: Json<technical::Base64Input>) -> Json<technical::Base64Result> {
    Json(technical::encode_base64(input.into_inner()))
}

#[post("/base64/decode", format = "json", data = "<input>")]
pub fn base64_decode(input: Json<technical::Base64Input>) -> Json<technical::Base64Result> {
    Json(technical::decode_base64(input.into_inner()))
}

#[post("/json/format", format = "json", data = "<input>")]
pub fn json_format(input: Json<technical::JsonFormatInput>) -> Json<technical::JsonFormatResult> {
    Json(technical::format_json(input.into_inner()))
}

#[post("/epoch", format = "json", data = "<input>")]
pub fn epoch_calc(input: Json<technical::EpochInput>) -> Json<technical::EpochResult> {
    Json(technical::convert_epoch(input.into_inner()))
}

#[post("/business-days", format = "json", data = "<input>")]
pub fn business_days_calc(
    input: Json<technical::BusinessDaysInput>,
) -> Json<technical::BusinessDaysResult> {
    Json(technical::calculate_business_days(input.into_inner()))
}

#[post("/age", format = "json", data = "<input>")]
pub fn age_calc(input: Json<technical::AgeInput>) -> Json<technical::AgeResult> {
    Json(technical::calculate_age(input.into_inner()))
}

#[post("/password", format = "json", data = "<input>")]
pub fn password_calc(input: Json<technical::PasswordInput>) -> Json<technical::PasswordResult> {
    Json(technical::generate_password(input.into_inner()))
}

#[post("/timezone", format = "json", data = "<input>")]
pub fn timezone_calc(input: Json<technical::TimeZoneInput>) -> Json<technical::TimeZoneResult> {
    Json(technical::convert_time_zone(input.into_inner()))
}

#[post("/concrete", format = "json", data = "<input>")]
pub fn concrete_calc(input: Json<technical::ConcreteInput>) -> Json<technical::ConcreteResult> {
    Json(technical::estimate_concrete(input.into_inner()))
}

#[post("/tiles", format = "json", data = "<input>")]
pub fn tile_calc(input: Json<technical::TileInput>) -> Json<technical::TileResult> {
    Json(technical::estimate_tiles(input.into_inner()))
}

#[derive(Debug, serde::Deserialize)]
pub struct PercentageInput {
    pub value: f64,
    pub percentage: f64,
    pub calculation_type: PercentageType,
}

#[derive(Debug, serde::Deserialize)]
pub enum PercentageType {
    WhatIsXPercentOfY,
    XIsWhatPercentOfY,
    PercentageChange,
}

#[derive(Debug, serde::Serialize)]
pub struct PercentageResult {
    pub result: f64,
    pub formula: String,
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
            let result = (input.value / input.percentage) * 100.0;
            PercentageResult {
                result,
                formula: format!("{} is {}% of {}", input.value, result, input.percentage),
            }
        }
        PercentageType::PercentageChange => {
            let result = ((input.value - input.percentage) / input.percentage.abs()) * 100.0;
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
