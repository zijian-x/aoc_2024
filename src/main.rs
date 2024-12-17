mod day4;

use chrono::prelude::*;
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let _day = Utc::now().day();
    let _day = 4;
    let input_location = format!("data/{}/input", _day);
    let example_location = format!("data/{}/example", _day);
    let _input = File::open(&Path::new(&input_location))?;
    let _example = File::open(&Path::new(&example_location))?;

    // day4::part1::run(&_example);
    day4::part1::run(&_input);
    // day4::part2::run(&_example);
    // day4::part2::run(&input);

    Ok(())
}
