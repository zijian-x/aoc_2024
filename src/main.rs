mod day5;

use chrono::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let _day = Utc::now().day();
    let _day = 5;
    let _input = File::open(format!("data/{}/input", _day))?;
    let _example = File::open(format!("data/{}/example", _day))?;

    // day5::part1::run(_example);
    day5::part1::run(_input);
    // day5::part2::run(&_example);
    // day5::part2::run(&input);

    Ok(())
}
