use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickMathInput {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct QuickMathResult {
    pub result: f64,
    pub explanation: String,
}

pub fn calculate_quick_math(input: QuickMathInput) -> QuickMathResult {
    let normalized = input.query.to_lowercase();
    if let Some((percent, value)) = normalized.split_once("% of") {
        let percentage = percent.trim().parse::<f64>().unwrap_or(0.0);
        let base = value.trim().parse::<f64>().unwrap_or(0.0);
        let result = (percentage / 100.0) * base;
        return QuickMathResult {
            result,
            explanation: format!(
                "Take {}% of {} by multiplying {} x {}.",
                percentage,
                base,
                base,
                percentage / 100.0
            ),
        };
    }

    if let Some(value) = normalized.strip_prefix("root of ") {
        let number = value.trim().parse::<f64>().unwrap_or(0.0);
        return QuickMathResult {
            result: number.sqrt(),
            explanation: format!(
                "The square root of {} is the number that multiplies by itself to make {}.",
                number, number
            ),
        };
    }

    let sanitized = normalized.replace(' ', "");
    if let Some((left, right)) = sanitized.split_once('+') {
        let a = left.parse::<f64>().unwrap_or(0.0);
        let b = right.parse::<f64>().unwrap_or(0.0);
        return QuickMathResult {
            result: a + b,
            explanation: format!("Add {} and {}.", a, b),
        };
    }

    QuickMathResult {
        result: 0.0,
        explanation: "Try phrases like `20% of 500`, `root of 144`, or `4 + 6`.".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitConversionInput {
    pub value: f64,
    pub category: String,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct UnitConversionResult {
    pub base_unit: String,
    pub conversions: Vec<UnitValue>,
}

#[derive(Debug, Serialize)]
pub struct UnitValue {
    pub unit: String,
    pub value: f64,
}

pub fn convert_units(input: UnitConversionInput) -> UnitConversionResult {
    let category = input.category.to_lowercase();
    let unit = input.unit.to_lowercase();

    match category.as_str() {
        "speed" => {
            let meters_per_second_value = match unit.as_str() {
                "meter per second" | "meters per second" | "m/s" => input.value,
                "kilometer per hour" | "kilometers per hour" | "km/h" => input.value / 3.6,
                "mile per hour" | "miles per hour" | "mph" => input.value * 0.44704,
                "knot" | "knots" | "kt" => input.value * 0.514444,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "meter per second".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "meters per second".to_string(),
                        value: meters_per_second_value,
                    },
                    UnitValue {
                        unit: "kilometers per hour".to_string(),
                        value: meters_per_second_value * 3.6,
                    },
                    UnitValue {
                        unit: "miles per hour".to_string(),
                        value: meters_per_second_value / 0.44704,
                    },
                    UnitValue {
                        unit: "knots".to_string(),
                        value: meters_per_second_value / 0.514444,
                    },
                ],
            }
        }
        "data" => {
            let bytes_value = match unit.as_str() {
                "byte" | "bytes" | "b" => input.value,
                "kilobyte" | "kilobytes" | "kb" => input.value * 1024.0,
                "megabyte" | "megabytes" | "mb" => input.value * 1024.0 * 1024.0,
                "gigabyte" | "gigabytes" | "gb" => input.value * 1024.0 * 1024.0 * 1024.0,
                "terabyte" | "terabytes" | "tb" => input.value * 1024.0 * 1024.0 * 1024.0 * 1024.0,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "byte".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "bytes".to_string(),
                        value: bytes_value,
                    },
                    UnitValue {
                        unit: "kilobytes".to_string(),
                        value: bytes_value / 1024.0,
                    },
                    UnitValue {
                        unit: "megabytes".to_string(),
                        value: bytes_value / (1024.0 * 1024.0),
                    },
                    UnitValue {
                        unit: "gigabytes".to_string(),
                        value: bytes_value / (1024.0 * 1024.0 * 1024.0),
                    },
                    UnitValue {
                        unit: "terabytes".to_string(),
                        value: bytes_value / (1024.0 * 1024.0 * 1024.0 * 1024.0),
                    },
                ],
            }
        }
        "pressure" => {
            let pascal_value = match unit.as_str() {
                "pascal" | "pascals" | "pa" => input.value,
                "kilopascal" | "kilopascals" | "kpa" => input.value * 1000.0,
                "bar" => input.value * 100000.0,
                "psi" => input.value * 6894.76,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "pascal".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "pascals".to_string(),
                        value: pascal_value,
                    },
                    UnitValue {
                        unit: "kilopascals".to_string(),
                        value: pascal_value / 1000.0,
                    },
                    UnitValue {
                        unit: "bar".to_string(),
                        value: pascal_value / 100000.0,
                    },
                    UnitValue {
                        unit: "psi".to_string(),
                        value: pascal_value / 6894.76,
                    },
                ],
            }
        }
        "energy" => {
            let joule_value = match unit.as_str() {
                "joule" | "joules" | "j" => input.value,
                "kilojoule" | "kilojoules" | "kj" => input.value * 1000.0,
                "calorie" | "calories" | "cal" => input.value * 4.184,
                "kilowatt hour" | "kilowatt hours" | "kwh" => input.value * 3_600_000.0,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "joule".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "joules".to_string(),
                        value: joule_value,
                    },
                    UnitValue {
                        unit: "kilojoules".to_string(),
                        value: joule_value / 1000.0,
                    },
                    UnitValue {
                        unit: "calories".to_string(),
                        value: joule_value / 4.184,
                    },
                    UnitValue {
                        unit: "kilowatt hours".to_string(),
                        value: joule_value / 3_600_000.0,
                    },
                ],
            }
        }
        "volume" => {
            let liter_value = match unit.as_str() {
                "liter" | "liters" | "l" => input.value,
                "milliliter" | "milliliters" | "ml" => input.value / 1000.0,
                "gallon" | "gallons" | "gal" => input.value * 3.78541,
                "cup" | "cups" => input.value * 0.236588,
                "quart" | "quarts" => input.value * 0.946353,
                "pint" | "pints" => input.value * 0.473176,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "liter".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "liters".to_string(),
                        value: liter_value,
                    },
                    UnitValue {
                        unit: "milliliters".to_string(),
                        value: liter_value * 1000.0,
                    },
                    UnitValue {
                        unit: "gallons".to_string(),
                        value: liter_value / 3.78541,
                    },
                    UnitValue {
                        unit: "cups".to_string(),
                        value: liter_value / 0.236588,
                    },
                    UnitValue {
                        unit: "quarts".to_string(),
                        value: liter_value / 0.946353,
                    },
                    UnitValue {
                        unit: "pints".to_string(),
                        value: liter_value / 0.473176,
                    },
                ],
            }
        }
        "time" => {
            let seconds_value = match unit.as_str() {
                "second" | "seconds" | "sec" | "s" => input.value,
                "minute" | "minutes" | "min" => input.value * 60.0,
                "hour" | "hours" | "hr" | "h" => input.value * 3600.0,
                "day" | "days" => input.value * 86400.0,
                "week" | "weeks" => input.value * 604800.0,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "second".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "seconds".to_string(),
                        value: seconds_value,
                    },
                    UnitValue {
                        unit: "minutes".to_string(),
                        value: seconds_value / 60.0,
                    },
                    UnitValue {
                        unit: "hours".to_string(),
                        value: seconds_value / 3600.0,
                    },
                    UnitValue {
                        unit: "days".to_string(),
                        value: seconds_value / 86400.0,
                    },
                    UnitValue {
                        unit: "weeks".to_string(),
                        value: seconds_value / 604800.0,
                    },
                ],
            }
        }
        "area" => {
            let square_meter_value = match unit.as_str() {
                "square meter" | "square meters" | "sqm" | "m2" => input.value,
                "square foot" | "square feet" | "sqft" | "ft2" => input.value * 0.092903,
                "acre" | "acres" => input.value * 4046.86,
                "hectare" | "hectares" => input.value * 10000.0,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "square meter".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "square meters".to_string(),
                        value: square_meter_value,
                    },
                    UnitValue {
                        unit: "square feet".to_string(),
                        value: square_meter_value / 0.092903,
                    },
                    UnitValue {
                        unit: "acres".to_string(),
                        value: square_meter_value / 4046.86,
                    },
                    UnitValue {
                        unit: "hectares".to_string(),
                        value: square_meter_value / 10000.0,
                    },
                ],
            }
        }
        "weight" => {
            let kg_value = match unit.as_str() {
                "kilogram" | "kilograms" | "kg" => input.value,
                "gram" | "grams" | "g" => input.value / 1000.0,
                "pound" | "pounds" | "lb" | "lbs" => input.value * 0.453592,
                "ounce" | "ounces" | "oz" => input.value * 0.0283495,
                "tonne" | "tonnes" | "t" => input.value * 1000.0,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "kilogram".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "kilograms".to_string(),
                        value: kg_value,
                    },
                    UnitValue {
                        unit: "grams".to_string(),
                        value: kg_value * 1000.0,
                    },
                    UnitValue {
                        unit: "pounds".to_string(),
                        value: kg_value / 0.453592,
                    },
                    UnitValue {
                        unit: "ounces".to_string(),
                        value: kg_value / 0.0283495,
                    },
                    UnitValue {
                        unit: "tonnes".to_string(),
                        value: kg_value / 1000.0,
                    },
                ],
            }
        }
        "temperature" => {
            let celsius_value = match unit.as_str() {
                "celsius" | "c" => input.value,
                "fahrenheit" | "f" => (input.value - 32.0) * 5.0 / 9.0,
                "kelvin" | "k" => input.value - 273.15,
                _ => input.value,
            };
            UnitConversionResult {
                base_unit: "celsius".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "celsius".to_string(),
                        value: celsius_value,
                    },
                    UnitValue {
                        unit: "fahrenheit".to_string(),
                        value: (celsius_value * 9.0 / 5.0) + 32.0,
                    },
                    UnitValue {
                        unit: "kelvin".to_string(),
                        value: celsius_value + 273.15,
                    },
                ],
            }
        }
        _ => {
            let meter_value = match unit.as_str() {
                "meter" | "meters" | "m" => input.value,
                "millimeter" | "millimeters" | "mm" => input.value / 1000.0,
                "cm" | "centimeter" | "centimeters" => input.value / 100.0,
                "kilometer" | "kilometers" | "km" => input.value * 1000.0,
                "inch" | "inches" | "in" => input.value * 0.0254,
                "foot" | "feet" | "ft" => input.value * 0.3048,
                "yard" | "yards" | "yd" => input.value * 0.9144,
                "mile" | "miles" | "mi" => input.value * 1609.34,
                _ => input.value,
            };

            UnitConversionResult {
                base_unit: "meter".to_string(),
                conversions: vec![
                    UnitValue {
                        unit: "meters".to_string(),
                        value: meter_value,
                    },
                    UnitValue {
                        unit: "millimeters".to_string(),
                        value: meter_value * 1000.0,
                    },
                    UnitValue {
                        unit: "centimeters".to_string(),
                        value: meter_value * 100.0,
                    },
                    UnitValue {
                        unit: "kilometers".to_string(),
                        value: meter_value / 1000.0,
                    },
                    UnitValue {
                        unit: "inches".to_string(),
                        value: meter_value / 0.0254,
                    },
                    UnitValue {
                        unit: "feet".to_string(),
                        value: meter_value / 0.3048,
                    },
                    UnitValue {
                        unit: "yards".to_string(),
                        value: meter_value / 0.9144,
                    },
                    UnitValue {
                        unit: "miles".to_string(),
                        value: meter_value / 1609.34,
                    },
                ],
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpaInput {
    pub current_grade: f64,
    pub current_weight_percent: f64,
    pub desired_grade: f64,
    pub final_exam_weight_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct GpaResult {
    pub required_final_exam_score: f64,
    pub explanation: String,
}

pub fn calculate_gpa(input: GpaInput) -> GpaResult {
    let completed_weight = (100.0 - input.final_exam_weight_percent).max(0.0);
    let required_final_exam_score = ((input.desired_grade * 100.0)
        - (input.current_grade * completed_weight))
        / input.final_exam_weight_percent.max(1.0);

    GpaResult {
        required_final_exam_score,
        explanation: format!(
            "To finish with {:.1} overall, you need about {:.1} on the final exam.",
            input.desired_grade, required_final_exam_score
        ),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FractionInput {
    pub first_numerator: i32,
    pub first_denominator: i32,
    pub second_numerator: i32,
    pub second_denominator: i32,
    pub operation: String,
}

#[derive(Debug, Serialize)]
pub struct FractionResult {
    pub result_fraction: String,
    pub decimal_result: f64,
    pub steps: Vec<String>,
}

pub fn solve_fraction(input: FractionInput) -> FractionResult {
    let a = input.first_numerator;
    let b = input.first_denominator.max(1);
    let c = input.second_numerator;
    let d = input.second_denominator.max(1);

    let (num, den, step) = match input.operation.as_str() {
        "subtract" => (
            (a * d) - (c * b),
            b * d,
            "Find a common denominator and subtract.",
        ),
        "multiply" => (a * c, b * d, "Multiply numerators and denominators."),
        "divide" => (
            a * d,
            b * c.max(1),
            "Flip the second fraction and multiply.",
        ),
        _ => (
            (a * d) + (c * b),
            b * d,
            "Find a common denominator and add.",
        ),
    };

    let gcd = greatest_common_divisor(num.abs(), den.abs().max(1));
    let simple_num = num / gcd;
    let simple_den = den / gcd;

    FractionResult {
        result_fraction: format!("{}/{}", simple_num, simple_den),
        decimal_result: simple_num as f64 / simple_den as f64,
        steps: vec![
            format!("Start with {}/{} and {}/{}.", a, b, c, d),
            step.to_string(),
            format!("Simplify to {}/{}.", simple_num, simple_den),
        ],
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScientificInput {
    pub value: f64,
    pub second_value: Option<f64>,
    pub operation: String,
}

#[derive(Debug, Serialize)]
pub struct ScientificResult {
    pub result: f64,
    pub explanation: String,
}

pub fn calculate_scientific(input: ScientificInput) -> ScientificResult {
    let result = match input.operation.as_str() {
        "sin" => input.value.to_radians().sin(),
        "cos" => input.value.to_radians().cos(),
        "tan" => input.value.to_radians().tan(),
        "log10" => input.value.log10(),
        "ln" => input.value.ln(),
        "sqrt" => input.value.sqrt(),
        "pow" => input.value.powf(input.second_value.unwrap_or(2.0)),
        _ => input.value,
    };

    ScientificResult {
        result,
        explanation: format!("Applied {} to the current value.", input.operation),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EquationInput {
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, Serialize)]
pub struct EquationResult {
    pub solution: f64,
    pub steps: Vec<String>,
}

pub fn solve_equation(input: EquationInput) -> EquationResult {
    let solution = (-input.b) / input.a.max(1e-9);
    EquationResult {
        solution,
        steps: vec![
            format!("Start with {}x + {} = 0.", input.a, input.b),
            format!("Move {} to the other side.", input.b),
            format!("Divide by {} to get x = {}.", input.a, solution),
        ],
    }
}

fn greatest_common_divisor(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs().max(1)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphInput {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub min_x: i32,
    pub max_x: i32,
}

#[derive(Debug, Serialize)]
pub struct GraphResult {
    pub formula: String,
    pub points: Vec<(f64, f64)>,
    pub steps: Vec<String>,
}

pub fn graph_quadratic(input: GraphInput) -> GraphResult {
    let mut points = Vec::new();
    for x in input.min_x..=input.max_x {
        let x = x as f64;
        let y = (input.a * x * x) + (input.b * x) + input.c;
        points.push((x, y));
    }

    GraphResult {
        formula: format!("y = {}x^2 + {}x + {}", input.a, input.b, input.c),
        points,
        steps: vec![
            "Start with y = ax^2 + bx + c.".to_string(),
            "Substitute each x value into the expression.".to_string(),
            "Plot the resulting coordinate pairs.".to_string(),
        ],
    }
}
