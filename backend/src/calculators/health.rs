use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BMIInput {
    pub weight: f64,
    pub height: f64,
    pub unit_system: UnitSystem,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnitSystem {
    Metric,
    Imperial,
}

#[derive(Debug, Serialize)]
pub struct BMIResult {
    pub bmi: f64,
    pub category: String,
    pub healthy_weight_range: (f64, f64),
    pub risk_level: String,
}

pub fn calculate_bmi(input: BMIInput) -> BMIResult {
    let bmi = match input.unit_system {
        UnitSystem::Metric => {
            let height_m = input.height / 100.0;
            input.weight / (height_m * height_m)
        }
        UnitSystem::Imperial => {
            let height_inches = input.height;
            (input.weight * 703.0) / (height_inches * height_inches)
        }
    };

    let category = match bmi {
        b if b < 18.5 => "Underweight",
        b if b < 25.0 => "Normal weight",
        b if b < 30.0 => "Overweight",
        _ => "Obese",
    }
    .to_string();

    let risk_level = match bmi {
        b if b < 18.5 => "Increased risk of nutritional deficiency",
        b if b < 25.0 => "Low risk",
        b if b < 30.0 => "Increased risk of cardiovascular disease",
        _ => "High risk of serious health conditions",
    }
    .to_string();

    let healthy_weight_range = match input.unit_system {
        UnitSystem::Metric => {
            let height_m = input.height / 100.0;
            let min_weight = 18.5 * height_m * height_m;
            let max_weight = 24.9 * height_m * height_m;
            (min_weight, max_weight)
        }
        UnitSystem::Imperial => {
            let height_inches = input.height;
            let min_weight = 18.5 * height_inches * height_inches / 703.0;
            let max_weight = 24.9 * height_inches * height_inches / 703.0;
            (min_weight, max_weight)
        }
    };

    BMIResult {
        bmi,
        category,
        healthy_weight_range,
        risk_level,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TDEEInput {
    pub age: u32,
    pub gender: Gender,
    pub weight: f64,
    pub height: f64,
    pub activity_level: ActivityLevel,
    pub unit_system: UnitSystem,
    pub goal: Goal,
    pub target_weight: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,   // Little or no exercise
    Light,       // Exercise 1-3 times/week
    Moderate,    // Exercise 4-5 times/week
    Active,      // Daily exercise or intense exercise 3-4 times/week
    VeryActive,  // Intense exercise 6-7 times/week
    ExtraActive, // Very intense exercise daily/physical job
}

#[derive(Debug, Serialize)]
pub struct TDEResult {
    pub bmr: f64,
    pub tdee: f64,
    pub maintenance_calories: f64,
    pub weight_loss_calories: f64,
    pub weight_gain_calories: f64,
    pub recommended_calories: f64,
    pub macros: MacroRecommendation,
    pub goal_timeline_weeks: Option<f64>,
    pub weekly_weight_change: f64,
}

#[derive(Debug, Serialize)]
pub struct MacroRecommendation {
    pub protein: f64,
    pub carbs: f64,
    pub fat: f64,
    pub protein_percent: f64,
    pub carbs_percent: f64,
    pub fat_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Goal {
    Maintain,
    Cut,
    Bulk,
}

impl ActivityLevel {
    fn multiplier(&self) -> f64 {
        match self {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::Active => 1.725,
            ActivityLevel::VeryActive => 1.9,
            ActivityLevel::ExtraActive => 2.2,
        }
    }
}

pub fn calculate_tdee(input: TDEEInput) -> TDEResult {
    // Calculate BMR using Mifflin-St Jeor Equation
    let bmr = match input.unit_system {
        UnitSystem::Metric => match input.gender {
            Gender::Male => {
                (10.0 * input.weight) + (6.25 * input.height) - (5.0 * input.age as f64) + 5.0
            }
            Gender::Female => {
                (10.0 * input.weight) + (6.25 * input.height) - (5.0 * input.age as f64) - 161.0
            }
        },
        UnitSystem::Imperial => {
            let weight_kg = input.weight * 0.453592;
            let height_cm = input.height * 2.54;
            match input.gender {
                Gender::Male => {
                    (10.0 * weight_kg) + (6.25 * height_cm) - (5.0 * input.age as f64) + 5.0
                }
                Gender::Female => {
                    (10.0 * weight_kg) + (6.25 * height_cm) - (5.0 * input.age as f64) - 161.0
                }
            }
        }
    };

    let tdee = bmr * input.activity_level.multiplier();

    let (recommended_calories, weekly_weight_change, macro_split) = match input.goal {
        Goal::Maintain => (tdee, 0.0, (0.30, 0.40, 0.30)),
        Goal::Cut => ((tdee - 500.0).max(1200.0), -0.45, (0.40, 0.30, 0.30)),
        Goal::Bulk => (tdee + 300.0, 0.27, (0.30, 0.45, 0.25)),
    };

    let protein_grams = (recommended_calories * macro_split.0) / 4.0;
    let carbs_grams = (recommended_calories * macro_split.1) / 4.0;
    let fat_grams = (recommended_calories * macro_split.2) / 9.0;
    let goal_timeline_weeks = input.target_weight.and_then(|target| {
        let current_weight = match input.unit_system {
            UnitSystem::Metric => input.weight,
            UnitSystem::Imperial => input.weight * 0.453592,
        };
        if weekly_weight_change == 0.0 {
            None
        } else {
            let delta = target - current_weight;
            let weeks = delta / weekly_weight_change;
            if weeks.is_sign_positive() {
                Some(weeks.abs())
            } else {
                None
            }
        }
    });

    TDEResult {
        bmr,
        tdee,
        maintenance_calories: tdee,
        weight_loss_calories: tdee - 500.0,
        weight_gain_calories: tdee + 500.0,
        recommended_calories,
        macros: MacroRecommendation {
            protein: protein_grams,
            carbs: carbs_grams,
            fat: fat_grams,
            protein_percent: macro_split.0 * 100.0,
            carbs_percent: macro_split.1 * 100.0,
            fat_percent: macro_split.2 * 100.0,
        },
        goal_timeline_weeks,
        weekly_weight_change,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyMetricsInput {
    pub gender: Gender,
    pub height_cm: f64,
    pub neck_cm: f64,
    pub waist_cm: f64,
    pub hip_cm: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct BodyMetricsResult {
    pub body_fat_percent: f64,
    pub ideal_weight_range_kg: (f64, f64),
}

pub fn calculate_body_metrics(input: BodyMetricsInput) -> BodyMetricsResult {
    let body_fat_percent = match input.gender {
        Gender::Male => {
            495.0
                / (1.0324 - 0.19077 * (input.waist_cm - input.neck_cm).log10()
                    + 0.15456 * input.height_cm.log10())
                - 450.0
        }
        Gender::Female => {
            let hip = input.hip_cm.unwrap_or(input.waist_cm);
            495.0
                / (1.29579 - 0.35004 * (input.waist_cm + hip - input.neck_cm).log10()
                    + 0.22100 * input.height_cm.log10())
                - 450.0
        }
    };

    let height_m = input.height_cm / 100.0;
    BodyMetricsResult {
        body_fat_percent,
        ideal_weight_range_kg: (18.5 * height_m * height_m, 24.9 * height_m * height_m),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PregnancyInput {
    pub last_period: String,
}

#[derive(Debug, Serialize)]
pub struct PregnancyResult {
    pub due_date: String,
    pub current_week: i64,
    pub days_until_next_trimester: i64,
    pub next_trimester: String,
}

pub fn calculate_pregnancy(input: PregnancyInput) -> PregnancyResult {
    let lmp = chrono::NaiveDate::parse_from_str(&input.last_period, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Utc::now().date_naive());
    let today = chrono::Utc::now().date_naive();
    let due_date = lmp + chrono::Days::new(280);
    let current_week = ((today - lmp).num_days() / 7).max(0);
    let (threshold, next_trimester) = if current_week < 13 {
        (13 * 7, "Second trimester")
    } else if current_week < 27 {
        (27 * 7, "Third trimester")
    } else {
        (40 * 7, "Delivery window")
    };

    PregnancyResult {
        due_date: due_date.format("%Y-%m-%d").to_string(),
        current_week,
        days_until_next_trimester: (threshold - (today - lmp).num_days()).max(0),
        next_trimester: next_trimester.to_string(),
    }
}
