mod day7;

use chrono::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let _day = Utc::now().day();
    let _day = 7;
    let _input = File::open(format!("data/{}/input", _day))?;
    let _example = File::open(format!("data/{}/example", _day))?;

    // day7::part1::run(_example);
    day7::part1::run(_input);

    Ok(())
}
