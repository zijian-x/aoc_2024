use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file: &File) -> Vec<Vec<i32>> {
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|elem| elem.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

/// iterate over a window of 3, check the two differences and accumulate the order, or None if
/// differences are over the threshold of [1, 3]
fn determine_order(array: &[i32]) -> Option<(bool, bool)> {
    array.windows(3).fold(Some((true, true)), |acc, win| {
        if let Some((inc, dec)) = acc {
            let diff1 = win[0] - win[1];
            let diff2 = win[1] - win[2];
            if diff1.abs() < 1 || diff1.abs() > 3 || diff2.abs() < 1 || diff2.abs() > 3 {
                return None;
            }

            return Some((
                inc && diff1 < 0 && diff2 < 0,
                dec && diff1 > 0 && diff2 > 0,
            ));
        }

        None
    })
}

pub fn part1(file: &File) {
    let reports = parse(file);

    let count = reports.iter()
        .map(Vec::as_slice)
        .filter_map(determine_order) // JESUS CHRIST I LOVE RUST
        .filter(|(inc, dec)| inc != dec)
        .count();

    println!("{}", count);
}
