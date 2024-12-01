mod day1;

use chrono::prelude::*;
use std::path::Path;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let day = Utc::now().day();
    let input_location = format!("data/{}/input", day);
    let example_location = format!("data/{}/example", day);
    let input = File::open(&Path::new(&input_location))?;
    let example = File::open(&Path::new(&example_location))?;

    // day1::part1(&input);
    day1::part2(&input);

    Ok(())
}
