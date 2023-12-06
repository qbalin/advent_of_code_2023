use super::file_reader;
use std::collections::HashSet;

fn margin(distance: f64, time: f64) -> u64 {
    let min_press_time = ((time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0).ceil();
    let max_press_time = ((time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0).floor();
    println!("min_press_time: {}, max_press_time: {}", min_press_time, max_press_time);
    (1.0_f64 + max_press_time - min_press_time).round() as u64
}

// We are looking at the minimum pressing time required to win the race
// distance_done = press_time * (time - press_time)
// distance <= distance_done
// distance <= press_time * (time - press_time)
// min_press_time**2 - min_press_time* time + distance = 0
// min_press_time = (time - sqrt(time**2 - 4 * distance)) / 2
// Similarly, max_press_time = (time + sqrt(time**2 - 4 * distance)) / 2
pub fn run() -> Result<u64, std::io::Result<u64>> {
    Ok(margin(277.0_f64, 44.0_f64) * margin(1136.0_f64, 89.0_f64) * margin(1890.0_f64, 96.0_f64) * margin(1768.0_f64, 91.0_f64))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 2344708u64);
    }
}

