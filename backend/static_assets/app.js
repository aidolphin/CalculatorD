const historyKey = "calculator_history_v1";
const themeKey = "calculator_theme_v1";

const el = (id) => document.getElementById(id);

const setResult = (id, html, isError = false) => {
    const element = el(id);
    element.classList.remove("hidden", "error");
    if (isError) {
        element.classList.add("error");
    } else {
        element.classList.remove("error");
    }
    element.innerHTML = html;
};

const hideResult = (id) => el(id).classList.add("hidden");

const formatCurrency = (value) =>
    Number(value).toLocaleString(undefined, {
        style: "currency",
        currency: "USD",
        maximumFractionDigits: 2
    });

const round = (value, digits = 2) => Number(value).toFixed(digits);

const apiPost = async (path, payload) => {
    const response = await fetch(path, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload)
    });

    if (!response.ok) {
        throw new Error(`Request failed for ${path}`);
    }

    return response.json();
};

const withButtonLock = async (button, task) => {
    const originalLabel = button.textContent;
    button.disabled = true;
    button.textContent = "Working...";
    try {
        await task();
    } finally {
        button.disabled = false;
        button.textContent = originalLabel;
    }
};

const getHistory = () => {
    try {
        return JSON.parse(localStorage.getItem(historyKey) || "[]");
    } catch {
        return [];
    }
};

const saveHistoryEntry = (title, summary) => {
    const next = [{ title, summary, at: new Date().toLocaleString() }, ...getHistory()].slice(0, 12);
    localStorage.setItem(historyKey, JSON.stringify(next));
    renderHistory();
};

const renderHistory = () => {
    const list = el("history-list");
    const items = getHistory();
    if (!items.length) {
        list.innerHTML = "<div class='history-item'><strong>No history yet</strong><span>Your recent calculations will appear here.</span></div>";
        return;
    }

    list.innerHTML = items.map((item) => `
        <div class="history-item">
            <strong>${item.title}</strong>
            <span>${item.summary}</span>
            <span>${item.at}</span>
        </div>
    `).join("");
};

const setTheme = (mode) => {
    document.body.classList.toggle("dark-mode", mode === "dark");
    localStorage.setItem(themeKey, mode);
};

const unitOptions = {
    length: [
        ["meter", "Meter"],
        ["millimeter", "Millimeter"],
        ["cm", "Centimeter"],
        ["kilometer", "Kilometer"],
        ["inch", "Inch"],
        ["foot", "Foot"],
        ["yard", "Yard"],
        ["mile", "Mile"]
    ],
    weight: [
        ["kilogram", "Kilogram"],
        ["gram", "Gram"],
        ["pound", "Pound"],
        ["ounce", "Ounce"],
        ["tonne", "Tonne"]
    ],
    temperature: [
        ["celsius", "Celsius"],
        ["fahrenheit", "Fahrenheit"],
        ["kelvin", "Kelvin"]
    ],
    volume: [
        ["liter", "Liter"],
        ["milliliter", "Milliliter"],
        ["gallon", "Gallon"],
        ["cup", "Cup"],
        ["quart", "Quart"],
        ["pint", "Pint"]
    ],
    time: [
        ["second", "Second"],
        ["minute", "Minute"],
        ["hour", "Hour"],
        ["day", "Day"],
        ["week", "Week"]
    ],
    area: [
        ["square meter", "Square Meter"],
        ["square foot", "Square Foot"],
        ["acre", "Acre"],
        ["hectare", "Hectare"]
    ],
    speed: [
        ["meter per second", "Meter per Second"],
        ["kilometer per hour", "Kilometer per Hour"],
        ["mile per hour", "Mile per Hour"],
        ["knot", "Knot"]
    ],
    data: [
        ["byte", "Byte"],
        ["kilobyte", "Kilobyte"],
        ["megabyte", "Megabyte"],
        ["gigabyte", "Gigabyte"],
        ["terabyte", "Terabyte"]
    ],
    pressure: [
        ["pascal", "Pascal"],
        ["kilopascal", "Kilopascal"],
        ["bar", "Bar"],
        ["psi", "PSI"]
    ],
    energy: [
        ["joule", "Joule"],
        ["kilojoule", "Kilojoule"],
        ["calorie", "Calorie"],
        ["kilowatt hour", "Kilowatt Hour"]
    ]
};

const renderUnitOptions = () => {
    const category = el("unit-category").value;
    const select = el("unit-name");
    select.innerHTML = unitOptions[category]
        .map(([value, label]) => `<option value="${value}">${label}</option>`)
        .join("");
};

const drawMacroChart = (data) => {
    const chart = el("macro-chart");
    chart.classList.remove("hidden");
    const protein = data.macros.protein_percent;
    const carbs = data.macros.carbs_percent;
    const fat = data.macros.fat_percent;
    chart.innerHTML = `
        <div class="macro-visual" style="background: conic-gradient(#0f6adf 0 ${protein}%, #12a594 ${protein}% ${protein + carbs}%, #ffb020 ${protein + carbs}% 100%);"></div>
        <div class="macro-legend">
            <div><span class="swatch" style="background:#0f6adf;"></span>Protein ${round(data.macros.protein)}g (${round(protein, 0)}%)</div>
            <div><span class="swatch" style="background:#12a594;"></span>Carbs ${round(data.macros.carbs)}g (${round(carbs, 0)}%)</div>
            <div><span class="swatch" style="background:#ffb020;"></span>Fat ${round(data.macros.fat)}g (${round(fat, 0)}%)</div>
        </div>
    `;
};

const renderGraph = (points) => {
    const surface = el("graph-canvas");
    surface.classList.remove("hidden");
    const width = 500;
    const height = 220;
    const xs = points.map(([x]) => x);
    const ys = points.map(([, y]) => y);
    const minX = Math.min(...xs);
    const maxX = Math.max(...xs);
    const minY = Math.min(...ys);
    const maxY = Math.max(...ys);
    const mapX = (x) => ((x - minX) / (maxX - minX || 1)) * width;
    const mapY = (y) => height - ((y - minY) / (maxY - minY || 1)) * height;
    const path = points.map(([x, y], index) => `${index === 0 ? "M" : "L"} ${mapX(x)} ${mapY(y)}`).join(" ");
    const zeroX = minX <= 0 && maxX >= 0 ? mapX(0) : 0;
    const zeroY = minY <= 0 && maxY >= 0 ? mapY(0) : height;

    surface.innerHTML = `
        <svg viewBox="0 0 ${width} ${height}" role="img" aria-label="Quadratic graph">
            <line class="graph-axis" x1="0" y1="${zeroY}" x2="${width}" y2="${zeroY}"></line>
            <line class="graph-axis" x1="${zeroX}" y1="0" x2="${zeroX}" y2="${height}"></line>
            <path class="graph-line" d="${path}"></path>
        </svg>
    `;
};

const createRows = (rows) => `<div class="table-list">${rows.map(([label, value]) => `<div class="row"><strong>${label}</strong><span>${value}</span></div>`).join("")}</div>`;

const downloadReport = (title, html) => {
    const blob = new Blob([`<html><head><title>${title}</title></head><body style="font-family:Segoe UI,sans-serif;padding:24px;"><h1>${title}</h1>${html}</body></html>`], {
        type: "text/html"
    });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = `${title.toLowerCase().replace(/\s+/g, "-")}.html`;
    anchor.click();
    URL.revokeObjectURL(url);
};

const copyText = async (text) => {
    await navigator.clipboard.writeText(text);
};

const setDefaultDates = () => {
    const now = new Date();
    const iso = now.toISOString().slice(0, 10);
    const future = new Date(now.getTime() + 1000 * 60 * 60 * 24 * 75).toISOString().slice(0, 10);
    const pregnancy = new Date(now.getTime() - 1000 * 60 * 60 * 24 * 70).toISOString().slice(0, 10);
    el("business-start").value = iso;
    el("business-end").value = future;
    el("pregnancy-date").value = pregnancy;
    el("age-birth").value = "1998-01-01";
    el("age-compare").value = iso;
    el("timezone-datetime").value = new Date(now.getTime() + 1000 * 60 * 60).toISOString().slice(0, 16);
};

const buildMortgage = async () => {
    hideResult("mortgage-result");
    const termValue = Number(el("mortgage-term").value || 0);
    const termUnit = el("mortgage-term-unit").value;
    const rateType = el("mortgage-rate-type").value;
    const rate = Number(el("mortgage-rate").value || 0);
    const monthlyRateHint = rateType === "apr" ? `APR view selected at ${round(rate)}%` : `Fixed annual rate at ${round(rate)}%`;
    const payload = {
        loan_amount: Number(el("mortgage-loan").value || 0),
        interest_rate: rate,
        loan_term_years: termUnit === "years" ? termValue : Math.max(1, Math.round(termValue / 12)),
        loan_term_months: termUnit === "months" ? termValue : null,
        property_tax: Number(el("mortgage-tax").value || 0),
        home_insurance: Number(el("mortgage-insurance").value || 0),
        pmi: Number(el("mortgage-pmi").value || 0),
        down_payment: Number(el("mortgage-down").value || 0),
        extra_payment: Number(el("mortgage-extra").value || 0),
        annual_income: Number(el("mortgage-income").value || 0),
        filing_status: el("mortgage-status").value
    };
    const data = await apiPost("/api/mortgage", payload);

    const summaryRows = [
        ["Base Monthly Payment", formatCurrency(data.monthly_payment)],
        ["Total Housing Payment", formatCurrency(data.monthly_payment_with_housing)],
        ["Rate View", monthlyRateHint],
        ["Total Interest", formatCurrency(data.total_interest)],
        ["Interest Saved", formatCurrency(data.total_interest_saved)],
        ["Months Saved", data.months_saved],
        ["Payoff Date", data.payoff_date],
        ["Monthly Property Tax", formatCurrency(data.tax_summary.estimated_monthly_property_tax)],
        ["Monthly Home Insurance", formatCurrency(data.tax_summary.estimated_monthly_home_insurance)],
        ["Monthly Income Tax Estimate", formatCurrency(data.tax_summary.estimated_monthly_income_tax)],
        ["Effective Income Tax Rate", `${round(data.tax_summary.effective_income_tax_rate)}%`]
    ];
    const yearlyPreview = data.yearly_summary.slice(0, 5).map((year) =>
        `<li>Year ${year.year}: principal ${formatCurrency(year.principal_paid)}, interest ${formatCurrency(year.interest_paid)}, ending balance ${formatCurrency(year.ending_balance)}</li>`
    ).join("");

    const html = `
        ${createRows(summaryRows)}
        <h4>Yearly Amortization Preview</h4>
        <ul>${yearlyPreview}</ul>
    `;
    setResult("mortgage-result", html);
    saveHistoryEntry("Mortgage Planner", `${formatCurrency(data.monthly_payment_with_housing)} total housing payment`);
    el("mortgage-report").onclick = () => downloadReport("Mortgage Summary", html);
    el("mortgage-email").onclick = () => {
        const body = encodeURIComponent(`Mortgage summary\nMonthly payment: ${formatCurrency(data.monthly_payment)}\nTotal housing payment: ${formatCurrency(data.monthly_payment_with_housing)}\nPayoff date: ${data.payoff_date}`);
        window.location.href = `mailto:?subject=Mortgage Summary&body=${body}`;
    };
};

const buildCurrency = async () => {
    const data = await apiPost("/api/currency", {
        amount: Number(el("currency-amount").value || 0),
        from: el("currency-from").value,
        to: el("currency-to").value
    });
    setResult("currency-result", createRows([
        ["Converted", round(data.converted_amount, 4)],
        ["Rate", round(data.rate, 6)],
        ["Source", data.source],
        ["Fetched At", data.fetched_at]
    ]));
    saveHistoryEntry("Currency Converter", `${round(data.converted_amount, 2)} converted at rate ${round(data.rate, 4)}`);
};

const buildCompound = async () => {
    const data = await apiPost("/api/compound-interest", {
        principal: Number(el("compound-principal").value || 0),
        monthly_contribution: Number(el("compound-monthly").value || 0),
        annual_rate: Number(el("compound-rate").value || 0),
        years: Number(el("compound-years").value || 0),
        compound_frequency: el("compound-frequency").value
    });
    const finalYear = data.yearly_breakdown.at(-1);
    setResult("compound-result", createRows([
        ["Final Amount", formatCurrency(data.final_amount)],
        ["Total Contributions", formatCurrency(data.total_contributions)],
        ["Interest Earned", formatCurrency(data.total_interest)],
        ["Last Year Balance", finalYear ? formatCurrency(finalYear.balance) : formatCurrency(data.final_amount)]
    ]));
    saveHistoryEntry("Compound Interest", `${formatCurrency(data.final_amount)} projected balance`);
};

const buildInflation = async () => {
    const data = await apiPost("/api/inflation", {
        amount: Number(el("inflation-amount").value || 0),
        from_year: Number(el("inflation-from-year").value || 0),
        to_year: Number(el("inflation-to-year").value || 0),
        annual_inflation_rate: Number(el("inflation-rate").value || 0)
    });
    setResult("inflation-result", createRows([
        ["Starting Value", formatCurrency(data.present_value)],
        ["Future Value", formatCurrency(data.future_value)],
        ["Inflation Difference", formatCurrency(data.total_inflation)],
        ["Years Compared", data.yearly_values.length ? `${data.yearly_values.length - 1} years` : "0 years"]
    ]));
    saveHistoryEntry("Inflation Calculator", `${formatCurrency(data.future_value)} estimated value`);
};

const buildInvestment = async () => {
    const data = await apiPost("/api/investment", {
        monthly_investment: Number(el("investment-monthly").value || 0),
        annual_return_rate: Number(el("investment-rate").value || 0),
        years: Number(el("investment-years").value || 0),
        initial_investment: Number(el("investment-initial").value || 0)
    });
    setResult("investment-result", createRows([
        ["Final Value", formatCurrency(data.final_value)],
        ["Total Contributions", formatCurrency(data.total_contributions)],
        ["Total Growth", formatCurrency(data.total_growth)]
    ]));
    saveHistoryEntry("Investment Planner", `${formatCurrency(data.final_value)} projected value`);
};

const buildBmi = async () => {
    const data = await apiPost("/api/bmi", {
        weight: Number(el("bmi-weight").value || 0),
        height: Number(el("bmi-height").value || 0),
        unit_system: "Metric"
    });
    setResult("bmi-result", createRows([
        ["BMI", round(data.bmi, 2)],
        ["Category", data.category],
        ["Healthy Weight Range", `${round(data.healthy_weight_range[0])}kg - ${round(data.healthy_weight_range[1])}kg`],
        ["Risk", data.risk_level]
    ]));
    saveHistoryEntry("BMI Calculator", `BMI ${round(data.bmi, 1)} ${data.category}`);
};

const buildTdee = async () => {
    const data = await apiPost("/api/tdee", {
        age: Number(el("tdee-age").value || 0),
        gender: el("tdee-gender").value,
        weight: Number(el("tdee-weight").value || 0),
        height: Number(el("tdee-height").value || 0),
        activity_level: el("tdee-activity").value,
        unit_system: "Metric",
        goal: el("tdee-goal").value,
        target_weight: Number(el("tdee-target").value || 0)
    });
    drawMacroChart(data);
    setResult("tdee-result", createRows([
        ["BMR", `${round(data.bmr)} kcal`],
        ["TDEE", `${round(data.tdee)} kcal`],
        ["Recommended Calories", `${round(data.recommended_calories)} kcal`],
        ["Weekly Weight Change", `${round(data.weekly_weight_change, 2)} kg`],
        ["Goal Timeline", data.goal_timeline_weeks ? `${round(data.goal_timeline_weeks, 1)} weeks` : "Not available"]
    ]));
    saveHistoryEntry("TDEE & Macro Coach", `${round(data.recommended_calories)} kcal with ${round(data.macros.protein)}g protein`);
};

const buildBodyMetrics = async () => {
    const data = await apiPost("/api/body-metrics", {
        gender: el("body-gender").value,
        height_cm: Number(el("body-height").value || 0),
        neck_cm: Number(el("body-neck").value || 0),
        waist_cm: Number(el("body-waist").value || 0),
        hip_cm: Number(el("body-hip").value || 0)
    });
    setResult("body-result", createRows([
        ["Body Fat %", `${round(data.body_fat_percent)}%`],
        ["Ideal Weight Range", `${round(data.ideal_weight_range_kg[0])}kg - ${round(data.ideal_weight_range_kg[1])}kg`]
    ]));
    saveHistoryEntry("Body Metrics", `${round(data.body_fat_percent)}% body fat estimate`);
};

const buildPregnancy = async () => {
    const data = await apiPost("/api/pregnancy", {
        last_period: el("pregnancy-date").value
    });
    setResult("pregnancy-result", createRows([
        ["Due Date", data.due_date],
        ["Current Week", data.current_week],
        ["Next Milestone", data.next_trimester],
        ["Days Until Next Trimester", data.days_until_next_trimester]
    ]));
    saveHistoryEntry("Pregnancy Milestones", `Due ${data.due_date}`);
};

const buildQuickMath = async () => {
    const data = await apiPost("/api/quick-math", { query: el("quick-math-query").value });
    setResult("quick-math-result", createRows([
        ["Result", round(data.result, 4)],
        ["How It Works", data.explanation]
    ]));
    saveHistoryEntry("Quick-Type Math", `${el("quick-math-query").value} = ${round(data.result, 4)}`);
};

const buildPercentage = async () => {
    const data = await apiPost("/api/percentage", {
        value: Number(el("percentage-value").value || 0),
        percentage: Number(el("percentage-base").value || 0),
        calculation_type: el("percentage-type").value
    });
    setResult("percentage-result", createRows([
        ["Result", round(data.result, 4)],
        ["Formula", data.formula]
    ]));
    saveHistoryEntry("Percentage Calculator", data.formula);
};

const buildUnitMatrix = async () => {
    const data = await apiPost("/api/unit-convert", {
        value: Number(el("unit-value").value || 0),
        category: el("unit-category").value,
        unit: el("unit-name").value
    });
    setResult("unit-result", createRows(data.conversions.map((entry) => [entry.unit, round(entry.value, 4)])));
    saveHistoryEntry("Unit Conversion Matrix", `${el("unit-value").value} ${el("unit-name").value}`);
};

const buildScientific = async () => {
    const data = await apiPost("/api/scientific", {
        value: Number(el("scientific-value").value || 0),
        second_value: Number(el("scientific-second").value || 0),
        operation: el("scientific-operation").value
    });
    setResult("scientific-result", createRows([
        ["Result", round(data.result, 6)],
        ["Explanation", data.explanation]
    ]));
    saveHistoryEntry("Scientific Calculator", `${el("scientific-operation").value} result ${round(data.result, 4)}`);
};

const buildGpa = async () => {
    const data = await apiPost("/api/gpa", {
        current_grade: Number(el("gpa-current").value || 0),
        current_weight_percent: Number(el("gpa-current-weight").value || 0),
        desired_grade: Number(el("gpa-desired").value || 0),
        final_exam_weight_percent: Number(el("gpa-final-weight").value || 0)
    });
    setResult("gpa-result", createRows([
        ["Required Final Score", round(data.required_final_exam_score, 2)],
        ["Explanation", data.explanation]
    ]));
    saveHistoryEntry("GPA Calculator", `${round(data.required_final_exam_score, 1)} needed on final`);
};

const buildFraction = async () => {
    const data = await apiPost("/api/fraction", {
        first_numerator: Number(el("fraction-first-num").value || 0),
        first_denominator: Number(el("fraction-first-den").value || 1),
        second_numerator: Number(el("fraction-second-num").value || 0),
        second_denominator: Number(el("fraction-second-den").value || 1),
        operation: el("fraction-operation").value
    });
    setResult("fraction-result", `
        ${createRows([
            ["Result Fraction", data.result_fraction],
            ["Decimal", round(data.decimal_result, 4)]
        ])}
        <ul>${data.steps.map((step) => `<li>${step}</li>`).join("")}</ul>
    `);
    saveHistoryEntry("Fraction Solver", data.result_fraction);
};

const buildEquation = async () => {
    const data = await apiPost("/api/equation", {
        a: Number(el("equation-a").value || 0),
        b: Number(el("equation-b").value || 0)
    });
    setResult("equation-result", `
        ${createRows([["Solution", round(data.solution, 4)]])}
        <ul>${data.steps.map((step) => `<li>${step}</li>`).join("")}</ul>
    `);
    saveHistoryEntry("Equation Solver", `x = ${round(data.solution, 4)}`);
};

const buildGraph = async () => {
    const data = await apiPost("/api/graph", {
        a: Number(el("graph-a").value || 0),
        b: Number(el("graph-b").value || 0),
        c: Number(el("graph-c").value || 0),
        min_x: Number(el("graph-min-x").value || -5),
        max_x: Number(el("graph-max-x").value || 5)
    });
    renderGraph(data.points);
    setResult("graph-result", `<p><strong>${data.formula}</strong></p><ul>${data.steps.map((step) => `<li>${step}</li>`).join("")}</ul>`);
    saveHistoryEntry("Quadratic Graph", data.formula);
};

const buildBase64 = async (mode) => {
    const path = mode === "encode" ? "/api/base64/encode" : "/api/base64/decode";
    const data = await apiPost(path, { text: el("toolbox-text").value });
    setResult("toolbox-result", `<pre>${data.result}</pre><div class="button-row"><button id="copy-toolbox" type="button">Copy Result</button></div>`);
    el("copy-toolbox").onclick = () => copyText(data.result);
    saveHistoryEntry(`Base64 ${mode}`, data.result.slice(0, 36));
};

const buildJsonFormat = async () => {
    const data = await apiPost("/api/json/format", { text: el("toolbox-text").value });
    setResult("toolbox-result", `<pre>${data.pretty}</pre><div class="button-row"><button id="copy-toolbox" type="button">Copy Result</button></div>`);
    el("copy-toolbox").onclick = () => copyText(data.pretty);
    saveHistoryEntry("JSON Formatter", "Formatted JSON");
};

const buildEpoch = async () => {
    const data = await apiPost("/api/epoch", { epoch: Number(el("epoch-input").value || 0) });
    setResult("date-tools-result", createRows([["ISO Time", data.iso]]));
    saveHistoryEntry("Epoch Converter", data.iso);
};

const buildBusinessDays = async () => {
    const data = await apiPost("/api/business-days", {
        start_date: el("business-start").value,
        end_date: el("business-end").value
    });
    setResult("date-tools-result", createRows([["Business Days", data.business_days]]));
    saveHistoryEntry("Business Days", `${data.business_days} work days`);
};

const buildAge = async () => {
    const data = await apiPost("/api/age", {
        birth_date: el("age-birth").value,
        comparison_date: el("age-compare").value
    });
    setResult("age-result", createRows([
        ["Years", data.years],
        ["Months", data.months],
        ["Days", data.days],
        ["Total Days", data.total_days]
    ]));
    saveHistoryEntry("Age Calculator", `${data.years} years old`);
};

const buildPassword = async () => {
    const data = await apiPost("/api/password", {
        length: Number(el("password-length").value || 16),
        include_uppercase: el("password-uppercase").value === "true",
        include_numbers: el("password-numbers").value === "true",
        include_symbols: el("password-symbols").value === "true"
    });
    setResult("password-result", `<pre>${data.password}</pre><div class="button-row"><button id="copy-password" type="button">Copy Password</button></div>`);
    el("copy-password").onclick = () => copyText(data.password);
    saveHistoryEntry("Password Generator", "Strong password created");
};

const buildTimezone = async () => {
    const data = await apiPost("/api/timezone", {
        datetime: el("timezone-datetime").value,
        from_timezone: el("timezone-from").value,
        to_timezone: el("timezone-to").value
    });
    setResult("timezone-result", createRows([["Converted Time", data.converted_datetime]]));
    saveHistoryEntry("Time Zone Converter", data.converted_datetime);
};

const buildConcrete = async () => {
    const data = await apiPost("/api/concrete", {
        length_ft: Number(el("concrete-length").value || 0),
        width_ft: Number(el("concrete-width").value || 0),
        depth_in: Number(el("concrete-depth").value || 0)
    });
    setResult("construction-result", createRows([
        ["Concrete Volume", `${round(data.cubic_feet)} cubic ft`],
        ["Concrete Needed", `${round(data.cubic_yards)} cubic yd`]
    ]));
    saveHistoryEntry("Concrete Estimator", `${round(data.cubic_yards)} cubic yards`);
};

const buildTiles = async () => {
    const data = await apiPost("/api/tiles", {
        floor_length_ft: Number(el("tile-floor-length").value || 0),
        floor_width_ft: Number(el("tile-floor-width").value || 0),
        tile_length_in: Number(el("tile-length").value || 0),
        tile_width_in: Number(el("tile-width").value || 0),
        waste_percent: Number(el("tile-waste").value || 0)
    });
    setResult("construction-result", createRows([["Tiles Needed", data.tiles_needed]]));
    saveHistoryEntry("Tile Estimator", `${data.tiles_needed} tiles`);
};

const bindAction = (id, task, targetId) => {
    const button = el(id);
    button.addEventListener("click", async () => {
        await withButtonLock(button, async () => {
            try {
                await task();
            } catch (error) {
                if (targetId) {
                    setResult(targetId, `<p>${error.message}</p>`, true);
                }
            }
        });
    });
}

const init = () => {
    setTheme(localStorage.getItem(themeKey) || "light");
    renderHistory();
    setDefaultDates();
    renderUnitOptions();

    el("theme-toggle").addEventListener("click", () => {
        setTheme(document.body.classList.contains("dark-mode") ? "light" : "dark");
    });
    el("nav-toggle").addEventListener("click", () => {
        el("top-nav").classList.toggle("is-open");
        el("app-header").classList.remove("is-hidden");
    });
    el("top-nav").querySelectorAll("a").forEach((link) => {
        link.addEventListener("click", () => {
            el("top-nav").classList.remove("is-open");
        });
    });
    el("clear-history").addEventListener("click", () => {
        localStorage.removeItem(historyKey);
        renderHistory();
    });
    el("unit-category").addEventListener("change", renderUnitOptions);

    bindAction("mortgage-submit", buildMortgage, "mortgage-result");
    bindAction("compound-submit", buildCompound, "compound-result");
    bindAction("inflation-submit", buildInflation, "inflation-result");
    bindAction("currency-submit", buildCurrency, "currency-result");
    bindAction("investment-submit", buildInvestment, "investment-result");
    bindAction("bmi-submit", buildBmi, "bmi-result");
    bindAction("tdee-submit", buildTdee, "tdee-result");
    bindAction("body-submit", buildBodyMetrics, "body-result");
    bindAction("pregnancy-submit", buildPregnancy, "pregnancy-result");
    bindAction("quick-math-submit", buildQuickMath, "quick-math-result");
    bindAction("percentage-submit", buildPercentage, "percentage-result");
    bindAction("unit-submit", buildUnitMatrix, "unit-result");
    bindAction("scientific-submit", buildScientific, "scientific-result");
    bindAction("gpa-submit", buildGpa, "gpa-result");
    bindAction("fraction-submit", buildFraction, "fraction-result");
    bindAction("equation-submit", buildEquation, "equation-result");
    bindAction("graph-submit", buildGraph, "graph-result");
    bindAction("base64-encode-submit", () => buildBase64("encode"), "toolbox-result");
    bindAction("base64-decode-submit", () => buildBase64("decode"), "toolbox-result");
    bindAction("json-format-submit", buildJsonFormat, "toolbox-result");
    bindAction("epoch-submit", buildEpoch, "date-tools-result");
    bindAction("business-submit", buildBusinessDays, "date-tools-result");
    bindAction("age-submit", buildAge, "age-result");
    bindAction("password-submit", buildPassword, "password-result");
    bindAction("timezone-submit", buildTimezone, "timezone-result");
    bindAction("concrete-submit", buildConcrete, "construction-result");
    bindAction("tile-submit", buildTiles, "construction-result");

    let lastScrollY = window.scrollY;
const updateHeaderState = () => {
        const header = el("app-header");
        const navOpen = el("top-nav").classList.contains("is-open");
        const currentScrollY = window.scrollY;
        const stickyActive = currentScrollY > 80;
        const scrollingDown = currentScrollY > lastScrollY;

        // Keep the header around when people need it, but get it out of the way during longer calculator sessions.
        header.classList.toggle("is-sticky", stickyActive);
        header.classList.toggle("is-hidden", stickyActive && scrollingDown && currentScrollY > 180 && !navOpen);

        lastScrollY = currentScrollY;
    };

    window.addEventListener("scroll", updateHeaderState, { passive: true });
    updateHeaderState();
}

document.addEventListener("DOMContentLoaded", init);
