use base64::{engine::general_purpose, Engine as _};
use chrono::{
    DateTime, Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc, Weekday,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64Input {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Base64Result {
    pub result: String,
}

pub fn encode_base64(input: Base64Input) -> Base64Result {
    Base64Result {
        result: general_purpose::STANDARD.encode(input.text),
    }
}

pub fn decode_base64(input: Base64Input) -> Base64Result {
    let decoded = general_purpose::STANDARD
        .decode(input.text)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_else(|| "Invalid Base64 input".to_string());
    Base64Result { result: decoded }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFormatInput {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct JsonFormatResult {
    pub pretty: String,
}

pub fn format_json(input: JsonFormatInput) -> JsonFormatResult {
    let pretty = serde_json::from_str::<serde_json::Value>(&input.text)
        .ok()
        .and_then(|value| serde_json::to_string_pretty(&value).ok())
        .unwrap_or_else(|| "Invalid JSON".to_string());
    JsonFormatResult { pretty }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpochInput {
    pub epoch: i64,
}

#[derive(Debug, Serialize)]
pub struct EpochResult {
    pub iso: String,
}

pub fn convert_epoch(input: EpochInput) -> EpochResult {
    let iso = DateTime::<Utc>::from_timestamp(input.epoch, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "Invalid epoch".to_string());
    EpochResult { iso }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessDaysInput {
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize)]
pub struct BusinessDaysResult {
    pub business_days: i64,
}

pub fn calculate_business_days(input: BusinessDaysInput) -> BusinessDaysResult {
    let start = NaiveDate::parse_from_str(&input.start_date, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());
    let end = NaiveDate::parse_from_str(&input.end_date, "%Y-%m-%d").unwrap_or(start);
    let mut cursor = start;
    let mut count = 0;

    while cursor <= end {
        if !matches!(cursor.weekday(), Weekday::Sat | Weekday::Sun) {
            count += 1;
        }
        cursor += Duration::days(1);
    }

    BusinessDaysResult {
        business_days: count,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConcreteInput {
    pub length_ft: f64,
    pub width_ft: f64,
    pub depth_in: f64,
}

#[derive(Debug, Serialize)]
pub struct ConcreteResult {
    pub cubic_feet: f64,
    pub cubic_yards: f64,
}

pub fn estimate_concrete(input: ConcreteInput) -> ConcreteResult {
    let cubic_feet = input.length_ft * input.width_ft * (input.depth_in / 12.0);
    ConcreteResult {
        cubic_feet,
        cubic_yards: cubic_feet / 27.0,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileInput {
    pub floor_length_ft: f64,
    pub floor_width_ft: f64,
    pub tile_length_in: f64,
    pub tile_width_in: f64,
    pub waste_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct TileResult {
    pub tiles_needed: u32,
}

pub fn estimate_tiles(input: TileInput) -> TileResult {
    let floor_sq_in = input.floor_length_ft * 12.0 * input.floor_width_ft * 12.0;
    let tile_sq_in = input.tile_length_in * input.tile_width_in;
    let raw_tiles = floor_sq_in / tile_sq_in;
    let adjusted = raw_tiles * (1.0 + input.waste_percent / 100.0);
    TileResult {
        tiles_needed: adjusted.ceil() as u32,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgeInput {
    pub birth_date: String,
    pub comparison_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AgeResult {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub total_days: i64,
}

pub fn calculate_age(input: AgeInput) -> AgeResult {
    let birth_date = NaiveDate::parse_from_str(&input.birth_date, "%Y-%m-%d")
        .unwrap_or_else(|_| Utc::now().date_naive());
    let comparison_date = input
        .comparison_date
        .and_then(|date| NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    let total_days = (comparison_date - birth_date).num_days().max(0);
    AgeResult {
        years: total_days / 365,
        months: (total_days % 365) / 30,
        days: (total_days % 365) % 30,
        total_days,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordInput {
    pub length: usize,
    pub include_uppercase: bool,
    pub include_numbers: bool,
    pub include_symbols: bool,
}

#[derive(Debug, Serialize)]
pub struct PasswordResult {
    pub password: String,
}

pub fn generate_password(input: PasswordInput) -> PasswordResult {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
    if input.include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if input.include_numbers {
        charset.push_str("0123456789");
    }
    if input.include_symbols {
        charset.push_str("!@#$%^&*()-_=+?");
    }

    let chars: Vec<char> = charset.chars().collect();
    let mut seed = Utc::now().timestamp_nanos_opt().unwrap_or_default() as usize;
    let mut password = String::new();
    for _ in 0..input.length.clamp(8, 64) {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        password.push(chars[seed % chars.len()]);
    }

    PasswordResult { password }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeZoneInput {
    pub datetime: String,
    pub from_timezone: String,
    pub to_timezone: String,
}

#[derive(Debug, Serialize)]
pub struct TimeZoneResult {
    pub converted_datetime: String,
}

pub fn convert_time_zone(input: TimeZoneInput) -> TimeZoneResult {
    let naive = NaiveDateTime::parse_from_str(&input.datetime, "%Y-%m-%dT%H:%M")
        .unwrap_or_else(|_| Utc::now().naive_utc());
    let from_offset = timezone_offset(&input.from_timezone);
    let to_offset = timezone_offset(&input.to_timezone);

    let from_dt = from_offset
        .from_local_datetime(&naive)
        .single()
        .unwrap_or_else(|| from_offset.from_utc_datetime(&naive));
    let utc_dt = from_dt.with_timezone(&Utc);
    let converted = utc_dt.with_timezone(&to_offset);

    TimeZoneResult {
        converted_datetime: converted.format("%Y-%m-%d %H:%M %:z").to_string(),
    }
}

fn timezone_offset(value: &str) -> FixedOffset {
    match value {
        "UTC" => FixedOffset::east_opt(0).unwrap(),
        "America/New_York" => FixedOffset::west_opt(5 * 3600).unwrap(),
        "Europe/London" => FixedOffset::east_opt(0).unwrap(),
        "Asia/Kathmandu" => FixedOffset::east_opt(5 * 3600 + 45 * 60).unwrap(),
        "Asia/Dubai" => FixedOffset::east_opt(4 * 3600).unwrap(),
        "Asia/Kolkata" => FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap(),
        "Asia/Tokyo" => FixedOffset::east_opt(9 * 3600).unwrap(),
        "Australia/Sydney" => FixedOffset::east_opt(10 * 3600).unwrap(),
        _ => FixedOffset::east_opt(0).unwrap(),
    }
}
