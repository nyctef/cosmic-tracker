use chrono::{DateTime, Duration, Utc};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EorzeanTime {
    hours: u64,
    minutes: u64,
}

impl EorzeanTime {
    pub fn new(hours: u64, minutes: u64) -> Self {
        EorzeanTime { hours, minutes }
    }

    /// Given a specific server time, convert it to Eorzean time.
    pub fn from_chrono_time(chrono_time: DateTime<Utc>) -> Self {
        let eorzean_multiplier = 3600.0 / 175.0;
        let eorzean_time = chrono_time.timestamp_millis() as f64 * eorzean_multiplier;
        let eorzean_seconds = (eorzean_time / 1000.0) as u64 % 86400;
        let hours = eorzean_seconds / 3600;
        let minutes = (eorzean_seconds % 3600) / 60;
        EorzeanTime { hours, minutes }
    }

    /// For this Eorzean time, find the next real-world server time that time will occur.
    pub fn find_next_chrono_time(&self) -> DateTime<Utc> {
        let now = Utc::now();
        let eorzean_multiplier = 3600.0 / 175.0;
        let eorzean_time = now.timestamp_millis() as f64 * eorzean_multiplier;
        let eorzean_seconds = (eorzean_time / 1000.0) as u64 % 86400;
        let current_eorzean_hours = eorzean_seconds / 3600;
        let current_eorzean_minutes = (eorzean_seconds % 3600) / 60;

        // Calculate the interval in Eorzean minutes
        let current_total_minutes = current_eorzean_hours * 60 + current_eorzean_minutes;
        let target_total_minutes = self.hours * 60 + self.minutes;

        // Adjust for the fact that an Eorzean day is 70 real-world minutes
        let eorzean_day_minutes = 1440; // Total Eorzean minutes in a day
        let interval_minutes = if target_total_minutes >= current_total_minutes {
            target_total_minutes - current_total_minutes
        } else {
            eorzean_day_minutes - (current_total_minutes - target_total_minutes)
        };

        // Convert Eorzean minutes to real-life seconds (70 real-world minutes per Eorzean day)
        let real_life_seconds =
            (interval_minutes as f64 * 70.0 * 60.0 / eorzean_day_minutes as f64) as i64;

        // Calculate the target local time
        now + Duration::seconds(real_life_seconds)
    }

    pub fn interval_until_chrono(&self) -> Duration {
        let now = Utc::now();
        let target_time = self.find_next_chrono_time();
        target_time - now
    }

    pub fn format_hhmm(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn plus_hours(&self, hours: u64) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + hours * 60;
        let new_hours = (total_minutes / 60) % 24;
        let new_minutes = total_minutes % 60;
        EorzeanTime::new(new_hours, new_minutes)
    }
}

impl PartialOrd for EorzeanTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_total_minutes = self.hours * 60 + self.minutes;
        let other_total_minutes = other.hours * 60 + other.minutes;
        self_total_minutes.partial_cmp(&other_total_minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: tests for other methods

    #[test]
    fn test_plus_hours() {
        let eorzean_time = EorzeanTime::new(23, 30);
        let new_time = eorzean_time.plus_hours(2);
        assert_eq!(EorzeanTime::new(1, 30), new_time);
    }
}
