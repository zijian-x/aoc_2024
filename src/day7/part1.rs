use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file: File) -> Vec<(u64, Vec<u64>)> {
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let (target, operands) = line.split_once(':').unwrap();

            let target = target.parse::<u64>().unwrap();
            let operands = operands
                .trim_start()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            return (target, operands);
        })
        .collect::<Vec<_>>()
}

fn backtrack(operands: &[u64], idx: usize, sum: u64, target: u64) -> u64 {
    if idx == operands.len() {
        return if sum == target { sum } else { 0 }
    }

    let res = backtrack(operands, idx + 1, sum + operands[idx], target);
    return if res != 0 {
        res
    } else {
        let sum = sum.max(1);
        backtrack(operands, idx + 1, sum * operands[idx], target)
    };
}

pub fn run(file: File) {
    let lines = parse(file);
    // lines.iter().for_each(|line| println!("{:?}", line));

    let sum = lines
        .iter()
        .map(|(target, operands)| backtrack(operands.as_slice(), 0, 0, *target))
        .sum::<u64>();
    println!("{}", sum);
}
