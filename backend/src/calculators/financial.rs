use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageInput {
    pub loan_amount: f64,
    pub interest_rate: f64,
    pub loan_term_years: u32,
    pub loan_term_months: Option<u32>,
    pub property_tax: Option<f64>,
    pub home_insurance: Option<f64>,
    pub pmi: Option<f64>,
    pub down_payment: Option<f64>,
    pub extra_payment: Option<f64>,
    pub annual_income: Option<f64>,
    pub filing_status: Option<FilingStatus>,
}

#[derive(Debug, Serialize)]
pub struct MortgageResult {
    pub monthly_payment: f64,
    pub monthly_payment_with_housing: f64,
    pub total_payment: f64,
    pub total_interest: f64,
    pub total_interest_saved: f64,
    pub months_saved: u32,
    pub payoff_date: String,
    pub amortization_schedule: Vec<AmortizationEntry>,
    pub yearly_summary: Vec<YearlyAmortization>,
    pub tax_summary: TaxSummary,
}

#[derive(Debug, Serialize)]
pub struct AmortizationEntry {
    pub month: u32,
    pub payment: f64,
    pub principal: f64,
    pub interest: f64,
    pub remaining_balance: f64,
}

#[derive(Debug, Serialize)]
pub struct YearlyAmortization {
    pub year: u32,
    pub principal_paid: f64,
    pub interest_paid: f64,
    pub ending_balance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum FilingStatus {
    Single,
    MarriedJoint,
    HeadOfHousehold,
}

#[derive(Debug, Serialize)]
pub struct TaxSummary {
    pub estimated_monthly_property_tax: f64,
    pub estimated_monthly_home_insurance: f64,
    pub estimated_monthly_income_tax: f64,
    pub effective_income_tax_rate: f64,
}

pub fn calculate_mortgage(input: MortgageInput) -> MortgageResult {
    let principal = (input.loan_amount - input.down_payment.unwrap_or(0.0)).max(0.0);
    let monthly_rate = input.interest_rate / 100.0 / 12.0;
    let num_payments = input.loan_term_months.unwrap_or(input.loan_term_years * 12);
    let extra_payment = input.extra_payment.unwrap_or(0.0).max(0.0);
    let monthly_property_tax = input.property_tax.unwrap_or(0.0) / 12.0;
    let monthly_home_insurance = input.home_insurance.unwrap_or(0.0) / 12.0;
    let monthly_pmi = input.pmi.unwrap_or(0.0);

    let monthly_payment = if monthly_rate > 0.0 {
        principal * monthly_rate * (1.0 + monthly_rate).powi(num_payments as i32)
            / ((1.0 + monthly_rate).powi(num_payments as i32) - 1.0)
    } else {
        principal / num_payments as f64
    };

    let mut remaining_balance = principal;
    let mut amortization = Vec::new();
    let mut yearly_summary = Vec::new();
    let mut year_principal = 0.0;
    let mut year_interest = 0.0;

    for month in 1..=num_payments {
        let interest = remaining_balance * monthly_rate;
        let principal_payment = (monthly_payment - interest + extra_payment).min(remaining_balance);
        remaining_balance = (remaining_balance - principal_payment).max(0.0);

        amortization.push(AmortizationEntry {
            month,
            payment: monthly_payment + extra_payment,
            principal: principal_payment,
            interest,
            remaining_balance,
        });

        year_principal += principal_payment;
        year_interest += interest;

        if month % 12 == 0 || remaining_balance <= 0.0 {
            yearly_summary.push(YearlyAmortization {
                year: ((month - 1) / 12) + 1,
                principal_paid: year_principal,
                interest_paid: year_interest,
                ending_balance: remaining_balance,
            });
            year_principal = 0.0;
            year_interest = 0.0;
        }

        if remaining_balance <= 0.0 {
            break;
        }
    }

    let actual_months = amortization.len() as u32;
    let total_interest = amortization.iter().map(|entry| entry.interest).sum::<f64>();
    let total_payment = amortization.iter().map(|entry| entry.payment).sum::<f64>();
    let baseline_total_interest = (monthly_payment * num_payments as f64) - principal;
    let total_interest_saved = (baseline_total_interest - total_interest).max(0.0);
    let months_saved = num_payments.saturating_sub(actual_months);
    let income = input.annual_income.unwrap_or(0.0);
    let tax_summary = estimate_tax_summary(
        income,
        input.filing_status.unwrap_or(FilingStatus::Single),
        monthly_property_tax,
        monthly_home_insurance,
    );

    let payoff_date = chrono::Utc::now()
        .checked_add_months(chrono::Months::new(actual_months))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();

    MortgageResult {
        monthly_payment,
        monthly_payment_with_housing: monthly_payment
            + extra_payment
            + monthly_property_tax
            + monthly_home_insurance
            + monthly_pmi,
        total_payment,
        total_interest,
        total_interest_saved,
        months_saved,
        payoff_date,
        amortization_schedule: amortization,
        yearly_summary,
        tax_summary,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompoundInterestInput {
    pub principal: f64,
    pub monthly_contribution: f64,
    pub annual_rate: f64,
    pub years: u32,
    pub compound_frequency: CompoundFrequency,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CompoundFrequency {
    Daily,
    Monthly,
    Quarterly,
    Annually,
}

impl CompoundFrequency {
    fn periods_per_year(&self) -> f64 {
        match self {
            CompoundFrequency::Daily => 365.0,
            CompoundFrequency::Monthly => 12.0,
            CompoundFrequency::Quarterly => 4.0,
            CompoundFrequency::Annually => 1.0,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CompoundInterestResult {
    pub final_amount: f64,
    pub total_contributions: f64,
    pub total_interest: f64,
    pub yearly_breakdown: Vec<YearlyBreakdown>,
}

#[derive(Debug, Serialize)]
pub struct YearlyBreakdown {
    pub year: u32,
    pub balance: f64,
    pub contributions: f64,
    pub interest_earned: f64,
}

pub fn calculate_compound_interest(input: CompoundInterestInput) -> CompoundInterestResult {
    let periods_per_year = input.compound_frequency.periods_per_year();
    let rate_per_period = input.annual_rate / 100.0 / periods_per_year;
    let total_periods = (input.years as f64 * periods_per_year) as u32;

    let mut balance = input.principal;
    let mut total_contributions = input.principal;
    let mut yearly_breakdown = Vec::new();
    let mut years_elapsed = 0;

    for period in 1..=total_periods {
        balance *= 1.0 + rate_per_period;
        if input.monthly_contribution > 0.0 {
            balance += input.monthly_contribution;
            total_contributions += input.monthly_contribution;
        }

        if period % (periods_per_year as u32) == 0 {
            years_elapsed += 1;

            if years_elapsed <= input.years {
                yearly_breakdown.push(YearlyBreakdown {
                    year: years_elapsed,
                    balance,
                    contributions: total_contributions,
                    interest_earned: balance - total_contributions,
                });
            }
        }
    }

    let total_interest = balance - total_contributions;

    CompoundInterestResult {
        final_amount: balance,
        total_contributions,
        total_interest,
        yearly_breakdown,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoLoanInput {
    pub vehicle_price: f64,
    pub down_payment: f64,
    pub trade_in_value: Option<f64>,
    pub loan_term_months: u32,
    pub interest_rate: f64,
    pub sales_tax_rate: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct AutoLoanResult {
    pub monthly_payment: f64,
    pub total_cost: f64,
    pub total_interest: f64,
    pub loan_amount: f64,
    pub sales_tax_amount: f64,
}

pub fn calculate_auto_loan(input: AutoLoanInput) -> AutoLoanResult {
    let trade_in = input.trade_in_value.unwrap_or(0.0);
    let base_loan_amount = input.vehicle_price - input.down_payment - trade_in;
    let sales_tax_amount = base_loan_amount * (input.sales_tax_rate.unwrap_or(0.0) / 100.0);
    let loan_amount = base_loan_amount + sales_tax_amount;

    let monthly_rate = input.interest_rate / 100.0 / 12.0;
    let monthly_payment = if monthly_rate > 0.0 {
        loan_amount * monthly_rate * (1.0 + monthly_rate).powi(input.loan_term_months as i32)
            / ((1.0 + monthly_rate).powi(input.loan_term_months as i32) - 1.0)
    } else {
        loan_amount / input.loan_term_months as f64
    };

    let total_cost = monthly_payment * input.loan_term_months as f64;
    let total_interest = total_cost - loan_amount;

    AutoLoanResult {
        monthly_payment,
        total_cost,
        total_interest,
        loan_amount,
        sales_tax_amount,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InflationInput {
    pub amount: f64,
    pub from_year: u32,
    pub to_year: u32,
    pub annual_inflation_rate: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct InflationResult {
    pub future_value: f64,
    pub present_value: f64,
    pub total_inflation: f64,
    pub yearly_values: Vec<YearlyInflation>,
}

#[derive(Debug, Serialize)]
pub struct YearlyInflation {
    pub year: u32,
    pub value: f64,
}

pub fn calculate_inflation(input: InflationInput) -> InflationResult {
    let years = (input.to_year - input.from_year) as i32;
    let inflation_rate = input.annual_inflation_rate.unwrap_or(3.0) / 100.0;

    let future_value = input.amount * (1.0 + inflation_rate).powi(years);
    let total_inflation = future_value - input.amount;

    let mut yearly_values = Vec::new();
    let mut current_value = input.amount;

    for year in input.from_year..=input.to_year {
        yearly_values.push(YearlyInflation {
            year,
            value: current_value,
        });
        current_value *= 1.0 + inflation_rate;
    }

    InflationResult {
        future_value,
        present_value: input.amount,
        total_inflation,
        yearly_values,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyInput {
    pub amount: f64,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize)]
pub struct CurrencyResult {
    pub converted_amount: f64,
    pub rate: f64,
    pub source: String,
    pub fetched_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentInput {
    pub monthly_investment: f64,
    pub annual_return_rate: f64,
    pub years: u32,
    pub initial_investment: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct InvestmentResult {
    pub final_value: f64,
    pub total_contributions: f64,
    pub total_growth: f64,
    pub yearly_projection: Vec<YearlyBreakdown>,
}

#[derive(Debug, Deserialize)]
struct ExchangeRateApiResponse {
    rates: HashMap<String, f64>,
}

pub async fn convert_currency(input: CurrencyInput) -> CurrencyResult {
    let from = input.from.to_uppercase();
    let to = input.to.to_uppercase();
    let url = format!("https://open.er-api.com/v6/latest/{from}");

    let fetched = reqwest::get(url).await;
    if let Ok(response) = fetched {
        if let Ok(body) = response.json::<ExchangeRateApiResponse>().await {
            if let Some(rate) = body.rates.get(&to) {
                return CurrencyResult {
                    converted_amount: input.amount * rate,
                    rate: *rate,
                    source: "open.er-api.com".to_string(),
                    fetched_at: chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
                };
            }
        }
    }

    let fallback_rate = fallback_rate(&from, &to);
    CurrencyResult {
        converted_amount: input.amount * fallback_rate,
        rate: fallback_rate,
        source: "fallback".to_string(),
        fetched_at: chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string(),
    }
}

pub fn calculate_investment(input: InvestmentInput) -> InvestmentResult {
    let mut balance = input.initial_investment.unwrap_or(0.0);
    let mut total_contributions = balance;
    let monthly_rate = input.annual_return_rate / 100.0 / 12.0;
    let total_months = input.years * 12;
    let mut yearly_projection = Vec::new();

    for month in 1..=total_months {
        balance *= 1.0 + monthly_rate;
        balance += input.monthly_investment;
        total_contributions += input.monthly_investment;

        if month % 12 == 0 {
            yearly_projection.push(YearlyBreakdown {
                year: month / 12,
                balance,
                contributions: total_contributions,
                interest_earned: balance - total_contributions,
            });
        }
    }

    InvestmentResult {
        final_value: balance,
        total_contributions,
        total_growth: balance - total_contributions,
        yearly_projection,
    }
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

fn estimate_tax_summary(
    annual_income: f64,
    filing_status: FilingStatus,
    monthly_property_tax: f64,
    monthly_home_insurance: f64,
) -> TaxSummary {
    if annual_income <= 0.0 {
        return TaxSummary {
            estimated_monthly_property_tax: monthly_property_tax,
            estimated_monthly_home_insurance: monthly_home_insurance,
            estimated_monthly_income_tax: 0.0,
            effective_income_tax_rate: 0.0,
        };
    }

    let brackets = match filing_status {
        FilingStatus::Single => vec![
            (11_600.0, 0.10),
            (47_150.0, 0.12),
            (100_525.0, 0.22),
            (191_950.0, 0.24),
            (243_725.0, 0.32),
            (609_350.0, 0.35),
        ],
        FilingStatus::MarriedJoint => vec![
            (23_200.0, 0.10),
            (94_300.0, 0.12),
            (201_050.0, 0.22),
            (383_900.0, 0.24),
            (487_450.0, 0.32),
            (731_200.0, 0.35),
        ],
        FilingStatus::HeadOfHousehold => vec![
            (16_550.0, 0.10),
            (63_100.0, 0.12),
            (100_500.0, 0.22),
            (191_950.0, 0.24),
            (243_700.0, 0.32),
            (609_350.0, 0.35),
        ],
    };

    let mut lower = 0.0;
    let mut tax = 0.0;
    for (upper, rate) in brackets {
        if annual_income <= lower {
            break;
        }
        let taxable = annual_income.min(upper) - lower;
        if taxable > 0.0 {
            tax += taxable * rate;
        }
        lower = upper;
    }
    if annual_income > lower {
        tax += (annual_income - lower) * 0.37;
    }

    TaxSummary {
        estimated_monthly_property_tax: monthly_property_tax,
        estimated_monthly_home_insurance: monthly_home_insurance,
        estimated_monthly_income_tax: tax / 12.0,
        effective_income_tax_rate: (tax / annual_income) * 100.0,
    }
}
