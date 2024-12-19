mod day6;

use chrono::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let _day = Utc::now().day();
    let _day = 6;
    let _input = File::open(format!("data/{}/input", _day))?;
    let _example = File::open(format!("data/{}/example", _day))?;

    // day6::part1::run(_example);
    day6::part1::run(_input);

    Ok(())
}
