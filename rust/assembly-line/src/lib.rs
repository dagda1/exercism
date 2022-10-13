// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

static CARS_PER_HOUR: u32 = 221;

fn percentage(n: u32, per: f64) -> f64 {
    return n as f64 * per * 0.01;
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars: u32 = CARS_PER_HOUR * speed as u32;
    
    return match speed {
        1 ..= 4 => cars as f64,
        5 ..= 8 => percentage(cars, 90.0),
        9 | 10 => percentage(cars, 77.0),
        _ => 0.0
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32;
}
