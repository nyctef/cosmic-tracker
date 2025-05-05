use chrono::{Duration, Utc};
use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Serialize)]
struct TableRow {
    cells: Vec<String>,
}

#[derive(Serialize)]
struct WeatherPrediction {
    weather: String,
    start_time: String,
    time_until: String,
}

#[wasm_bindgen]
pub fn get_table_data() -> JsValue {
    let data = vec![
        TableRow {
            cells: vec!["Row 1, Col 1".to_string(), "Row 1, Col 2".to_string()],
        },
        TableRow {
            cells: vec!["Row 2, Col 1".to_string(), "Row 2, Col 2".to_string()],
        },
    ];

    serde_wasm_bindgen::to_value(&data).unwrap()
}

#[wasm_bindgen]
pub fn get_weather_predictions() -> JsValue {
    let now = Utc::now();
    let mut predictions = Vec::new();
    let mut found = 0;
    let mut forecast_time = now;

    while found < 3 {
        forecast_time = forecast_time + Duration::seconds(8 * 175);
        let interval_start =
            forecast_time - Duration::seconds(forecast_time.timestamp() % (8 * 175));
        let weather = predict_weather(interval_start);

        if weather != "Fair Skies" {
            let time_until = interval_start - now;
            predictions.push(WeatherPrediction {
                weather: weather.to_string(),
                start_time: interval_start.format("%H:%M").to_string(),
                time_until: format_interval(time_until),
            });
            found += 1;
        }
    }

    serde_wasm_bindgen::to_value(&predictions).unwrap()
}

fn predict_weather(interval_start: chrono::DateTime<Utc>) -> &'static str {
    let forecast_target = calculate_forecast_target(interval_start);
    match forecast_target {
        0..=14 => "Moon Dust",
        15..=84 => "Fair Skies",
        85..=99 => "Umbral Wind",
        _ => unreachable!(),
    }
}

fn calculate_forecast_target(l_date: chrono::DateTime<Utc>) -> u32 {
    let unix_seconds = l_date.timestamp();
    let bell = unix_seconds / 175;
    let increment = (bell + 8 - (bell % 8)) % 24;
    let total_days = (unix_seconds / 4200) as u32;
    let calc_base = total_days.wrapping_mul(100).wrapping_add(increment as u32);
    let step1 = (calc_base << 11) ^ calc_base;
    let step2 = (step1 >> 8) ^ step1;
    step2 % 100
}

fn format_interval(d: Duration) -> String {
    let hours = d.num_hours();
    let minutes = d.num_minutes() % 60;
    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
