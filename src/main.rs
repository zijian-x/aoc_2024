mod day3;

use chrono::prelude::*;
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let day = Utc::now().day();
    let day = 3;
    let input_location = format!("data/{}/input", day);
    let example_location = format!("data/{}/example", day);
    let input = File::open(&Path::new(&input_location))?;
    let _example = File::open(&Path::new(&example_location))?;

    // day3::part1::run(&_example);
    day3::part1::run(&input);
    // day3::part2::run(&_example);
    // day3::part2::run(&input);

    Ok(())
}
