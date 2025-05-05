use chrono::{Duration, Local, Utc};
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
    local_time: String,
    time_until: String,
    alarm_macro: String,
}

#[derive(Serialize)]
pub struct TimeData {
    utc_time: String,
    local_time: String,
    eorzean_time: String,
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
            let alarm_macro = format!(
                "/alarm \"{} in 1m\" st {} 1 se00",
                weather,
                interval_start.format("%H%M")
            );
            let local_time = interval_start
                .with_timezone(&Local)
                .format("%H:%M")
                .to_string();
            predictions.push(WeatherPrediction {
                weather: weather.to_string(),
                start_time: interval_start.format("%H:%M").to_string(),
                local_time,
                time_until: format_interval(time_until),
                alarm_macro,
            });
            found += 1;
        }
    }

    serde_wasm_bindgen::to_value(&predictions).unwrap()
}

#[wasm_bindgen]
pub fn get_time_data() -> JsValue {
    let now = Utc::now();

    // UTC Time
    let utc_time = now.format("%H:%M:%S").to_string();

    // Local Time
    let local_time = Local::now().format("%H:%M:%S").to_string();

    // Eorzean Time (1 Eorzean hour = 175 seconds real time)
    let eorzean_multiplier = 3600.0 / 175.0;
    let eorzean_time = now.timestamp_millis() as f64 * eorzean_multiplier;
    let eorzean_seconds = (eorzean_time / 1000.0) as u64 % 86400;
    let eorzean_hours = eorzean_seconds / 3600;
    let eorzean_minutes = (eorzean_seconds % 3600) / 60;
    let eorzean_time = format!("{:02}:{:02}", eorzean_hours, eorzean_minutes);

    let time_data = TimeData {
        utc_time,
        local_time,
        eorzean_time,
    };

    serde_wasm_bindgen::to_value(&time_data).unwrap()
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
