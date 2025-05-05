mod eorzean_time;

use chrono::{Duration, Local, Utc};
use eorzean_time::EorzeanTime;
use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

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

#[derive(Serialize)]
struct MissionInfo {
    class_name: String,
    missions: Vec<String>,
    time_period: String,
    interval_until: String,
    next_local_time: String,
}

#[derive(Serialize)]
struct MissionData {
    class_name: String,
    missions: Vec<String>,
    time_period: String,
    target_hour: u64,
    target_minute: u64,
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

    let utc_time = now.format("%H:%M:%S").to_string();
    let local_time = Local::now().format("%H:%M:%S").to_string();
    let eorzean_time = EorzeanTime::from_chrono_time(now).format_hhmm();

    let time_data = TimeData {
        utc_time,
        local_time,
        eorzean_time,
    };

    serde_wasm_bindgen::to_value(&time_data).unwrap()
}

#[wasm_bindgen]
pub fn get_mission_schedule() -> JsValue {
    let mission_data = vec![
        MissionData {
            class_name: "Carpenter".to_string(),
            missions: vec!["High-durability Fiberboard".to_string()],
            time_period: "12:00 PM - 1:59 PM".to_string(),
            target_hour: 12,
            target_minute: 0,
        },
        MissionData {
            class_name: "Blacksmith".to_string(),
            missions: vec!["High-performance Drone Materials I".to_string()],
            time_period: "2:00 PM - 3:59 PM".to_string(),
            target_hour: 14,
            target_minute: 0,
        },
        MissionData {
            class_name: "Armorer".to_string(),
            missions: vec!["Starship Building Material Processing".to_string()],
            time_period: "4:00 PM - 5:59 PM".to_string(),
            target_hour: 16,
            target_minute: 0,
        },
        MissionData {
            class_name: "Goldsmith".to_string(),
            missions: vec!["Cosmotized Equipment Materials III".to_string()],
            time_period: "6:00 PM - 7:59 PM".to_string(),
            target_hour: 18,
            target_minute: 0,
        },
        MissionData {
            class_name: "Leatherworker".to_string(),
            missions: vec!["Cosmic Suits II".to_string()],
            time_period: "8:00 PM - 9:59 PM".to_string(),
            target_hour: 20,
            target_minute: 0,
        },
        MissionData {
            class_name: "Weaver".to_string(),
            missions: vec!["High-grade Composite Fiber".to_string()],
            time_period: "10:00 PM - 11:59 PM".to_string(),
            target_hour: 22,
            target_minute: 0,
        },
        MissionData {
            class_name: "Alchemist".to_string(),
            missions: vec!["Lunar Seafood Processing".to_string()],
            time_period: "12:00 AM - 1:59 AM".to_string(),
            target_hour: 0,
            target_minute: 0,
        },
        MissionData {
            class_name: "Culinarian".to_string(),
            missions: vec!["Nutrient Supplement Jelly".to_string()],
            time_period: "2:00 AM - 3:59 AM".to_string(),
            target_hour: 2,
            target_minute: 0,
        },
        MissionData {
            class_name: "Miner".to_string(),
            missions: vec!["Rare Siderite Extraction".to_string()],
            time_period: "4:00 AM - 5:59 AM".to_string(),
            target_hour: 4,
            target_minute: 0,
        },
        MissionData {
            class_name: "Botanist".to_string(),
            missions: vec!["Folkloric Materials III".to_string()],
            time_period: "6:00 AM - 7:59 AM".to_string(),
            target_hour: 6,
            target_minute: 0,
        },
        MissionData {
            class_name: "Fisher".to_string(),
            missions: vec!["Large Aquatic Specimens".to_string()],
            time_period: "8:00 AM - 9:59 AM".to_string(),
            target_hour: 8,
            target_minute: 0,
        },
    ];

    let schedule: Vec<MissionInfo> = mission_data
        .into_iter()
        .map(|data| {
            let target_time = EorzeanTime::new(data.target_hour, data.target_minute);
            MissionInfo {
                class_name: data.class_name,
                missions: data.missions,
                time_period: data.time_period,
                interval_until: format_interval(target_time.interval_until_chrono()),
                next_local_time: find_next_local_time(data.target_hour, data.target_minute),
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&schedule).unwrap()
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

pub fn find_next_local_time(eorzean_hour: u64, eorzean_minute: u64) -> String {
    let t = EorzeanTime::new(eorzean_hour, eorzean_minute);

    t.find_next_chrono_time()
        .with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
