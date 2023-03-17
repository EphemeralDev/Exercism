const BASE_HOURLY: f64 = 221.0;
const MINUTES_IN_HOUR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    BASE_HOURLY * speed as f64 * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / MINUTES_IN_HOUR
}

pub fn success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.90,
        9..=u8::MAX => 0.77,
    }
}
