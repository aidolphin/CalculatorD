# CalculatorD

CalculatorD is a practical calculator website built by Quantum Leaf Automation.
It puts money, health, math, and utility tools in one place so people can solve real problems quickly on desktop or mobile.

## What is included

- Mortgage planner with PMI, tax, insurance, extra payments, and amortization preview
- Compound interest, inflation, currency conversion, and investment planning
- BMI, TDEE, macros, body-fat estimate, and pregnancy milestones
- Percentage, quick math, scientific tools, GPA, fractions, equations, graphs, and unit conversion
- Age, business days, time zone conversion, password generation, JSON/Base64 tools, and construction estimators

## Run locally

```bash
cargo run -p backend
```

Open `http://127.0.0.1:8000`.

## Deploy notes

This project serves the static frontend and the Rust API from the same Rocket app.

Important environment variables:

- `ROCKET_ADDRESS=0.0.0.0`
- `ROCKET_PORT=8000`
- `ROCKET_SECRET_KEY=...`
- `SITE_URL=https://calculatord.dolphinsagar9.workers.dev/`

In debug mode the app now uses a stable development secret key automatically, so the Rocket warning is gone locally.
In release mode you should always set a real `ROCKET_SECRET_KEY`.

Example release command:

```bash
ROCKET_ADDRESS=0.0.0.0 \
ROCKET_PORT=8000 \
SITE_URL=https://your-domain.com \
ROCKET_SECRET_KEY=replace-with-a-real-secret \
cargo run --release -p backend
```

There is also a `Dockerfile` for container deployment.

## SEO and growth work already added

- Search-friendly page title and meta description
- `robots` meta tag
- Open Graph and Twitter metadata
- JSON-LD structured data
- `site.webmanifest`
- `/robots.txt`
- `/sitemap.xml`
- `/health` for uptime checks

## Search and marketing reality

Good SEO improves your chance to rank, but no codebase can honestly guarantee first place in search results.
What this project now gives you is a stronger on-page foundation:

- clear headings and category structure
- mobile-friendly layout
- crawlable metadata
- fast single-domain delivery
- practical tools with repeat-use value

To turn this into a real traffic and earning engine, the next product steps should be:

- publish dedicated landing pages for high-intent calculators like mortgage, BMI, TDEE, percentage, and age
- add original explanation content and FAQs around each tool
- connect analytics and search console
- add conversion points like newsletter signup, calculator comparison pages, or partner offers
- add ad and affiliate placements carefully without making the UI feel cheap

## Project structure

- `backend/` contains Rocket routes and calculator logic
- `static/` contains the production frontend that the backend serves
- `frontend/` is an older Rust frontend workspace member and is not the active UI path right now

## Quality checks

Run these before shipping changes:

```bash
cargo fmt
cargo check
cargo clippy --workspace --all-targets -- -D warnings
```

## Maintenance note

The code is being kept intentionally readable.
Short comments are only used where they help explain behavior that would otherwise take a second look.
