use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::{Match, Regex};

fn parse(file: &File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect()
}

pub fn run(file: &File) {
    let lines = parse(file);
    let rg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum = 0;

    for line in lines {
        for (s, [x, y]) in rg.captures_iter(&line).map(|c| c.extract()) {
            sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}
