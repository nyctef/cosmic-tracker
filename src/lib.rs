use chrono::{Duration, Local, Utc};
use serde::Serialize;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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

#[wasm_bindgen]
pub fn get_mission_schedule() -> JsValue {
    let now = Utc::now();
    let eorzean_multiplier = 3600.0 / 175.0;
    let eorzean_time = now.timestamp_millis() as f64 * eorzean_multiplier;
    let eorzean_seconds = (eorzean_time / 1000.0) as u64 % 86400;
    let eorzean_hours = eorzean_seconds / 3600;
    let eorzean_minutes = (eorzean_seconds % 3600) / 60;

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
        .map(|data| MissionInfo {
            class_name: data.class_name,
            missions: data.missions,
            time_period: data.time_period,
            interval_until: calculate_interval(eorzean_hours, eorzean_minutes, data.target_hour, data.target_minute),
            next_local_time: find_next_local_time(data.target_hour, data.target_minute),
        })
        .collect();

    serde_wasm_bindgen::to_value(&schedule).unwrap()
}

fn calculate_interval(
    current_hour: u64,
    current_minute: u64,
    target_hour: u64,
    target_minute: u64,
) -> String {
    let current_total_minutes = current_hour * 60 + current_minute;
    let target_total_minutes = target_hour * 60 + target_minute;

    let interval_minutes = if target_total_minutes >= current_total_minutes {
        target_total_minutes - current_total_minutes
    } else {
        1440 - (current_total_minutes - target_total_minutes)
    };

    // Convert Eorzean minutes to real-life minutes (70 real-world minutes per Eorzean day)
    let real_life_minutes = (interval_minutes as f64 * 70.0 / 1440.0).round() as u64;
    let hours = real_life_minutes / 60;
    let minutes = real_life_minutes % 60;

    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
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
    let now = Utc::now();
    let eorzean_multiplier = 3600.0 / 175.0;

    // Current Eorzean time
    let eorzean_time = now.timestamp_millis() as f64 * eorzean_multiplier;
    let eorzean_seconds = (eorzean_time / 1000.0) as u64 % 86400;
    let current_eorzean_hours = eorzean_seconds / 3600;
    let current_eorzean_minutes = (eorzean_seconds % 3600) / 60;

    // Calculate the interval in Eorzean minutes
    let current_total_minutes = current_eorzean_hours * 60 + current_eorzean_minutes;
    let target_total_minutes = eorzean_hour * 60 + eorzean_minute;

    // Adjust for the fact that an Eorzean day is 70 real-world minutes
    let eorzean_day_minutes = 1440; // Total Eorzean minutes in a day
    let interval_minutes = if target_total_minutes >= current_total_minutes {
        target_total_minutes - current_total_minutes
    } else {
        eorzean_day_minutes - (current_total_minutes - target_total_minutes)
    };

    // Convert Eorzean minutes to real-life seconds (70 real-world minutes per Eorzean day)
    let real_life_seconds = (interval_minutes as f64 * 70.0 * 60.0 / eorzean_day_minutes as f64) as i64;

    // Calculate the target local time
    let target_time = now + Duration::seconds(real_life_seconds);
    target_time.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string()
}
