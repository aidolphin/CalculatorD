#[macro_use]
extern crate rocket;

mod api;
mod calculators;

use rocket::figment::Figment;
use rocket::response::content::{RawText, RawXml};
use serde::Serialize;

const DEV_SECRET_KEY: &str = "7d63f62f93f54fb0b5e35c8f54d37de9f340c85d12a9a3cba69b9e8d4c1f27aa";

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

fn configured_figment() -> Figment {
    let figment = rocket::Config::figment();

    if std::env::var("ROCKET_SECRET_KEY").is_ok() || !cfg!(debug_assertions) {
        figment
    } else {
        figment.merge(("secret_key", DEV_SECRET_KEY))
    }
}

fn site_url() -> String {
    std::env::var("SITE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string())
        .trim_end_matches('/')
        .to_string()
}

#[get("/health")]
fn health_check() -> rocket::serde::json::Json<HealthResponse> {
    rocket::serde::json::Json(HealthResponse {
        status: "ok",
        service: "CalculatorD",
    })
}

#[get("/robots.txt")]
fn robots_txt() -> RawText<String> {
    RawText(format!(
        "User-agent: *\nAllow: /\n\nSitemap: {}/sitemap.xml\n",
        site_url()
    ))
}

#[get("/sitemap.xml")]
fn sitemap_xml() -> RawXml<String> {
    let base_url = site_url();
    RawXml(format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>{base_url}/</loc>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
</urlset>"#
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::custom(configured_figment())
        .mount(
            "/api",
            routes![
                api::routes::mortgage_calc,
                api::routes::compound_interest,
                api::routes::auto_loan,
                api::routes::inflation_calc,
                api::routes::bmi_calc,
                api::routes::tdee_calc,
                api::routes::body_metrics_calc,
                api::routes::pregnancy_calc,
                api::routes::percentage_calc,
                api::routes::quick_math_calc,
                api::routes::unit_convert,
                api::routes::graph_calc,
                api::routes::gpa_calc,
                api::routes::fraction_calc,
                api::routes::scientific_calc,
                api::routes::equation_calc,
                api::routes::currency_calc,
                api::routes::investment_calc,
                api::routes::base64_encode,
                api::routes::base64_decode,
                api::routes::json_format,
                api::routes::epoch_calc,
                api::routes::business_days_calc,
                api::routes::age_calc,
                api::routes::password_calc,
                api::routes::timezone_calc,
                api::routes::concrete_calc,
                api::routes::tile_calc,
            ],
        )
        .mount("/", routes![health_check, robots_txt, sitemap_xml])
        .mount("/", rocket::fs::FileServer::from("static"))
}
