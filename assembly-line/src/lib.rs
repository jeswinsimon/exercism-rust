// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: f64 = 221.;
const SUCCESS_RATE_HIGH: f64 = 1.;
const SUCCESS_RATE_MEDIUM: f64 = 0.9;
const SUCCESS_RATE_LOW: f64 = 0.77;
const MINS_IN_HR: u32 = 60;

fn success_rate(speed: u8) -> f64 {
    return match speed {
        1..=4 => SUCCESS_RATE_HIGH,
        5..=8 => SUCCESS_RATE_MEDIUM,
        _ => SUCCESS_RATE_LOW,
    };
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    return speed as f64 * CARS_PER_HOUR * success_rate(speed);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / MINS_IN_HR;
}
