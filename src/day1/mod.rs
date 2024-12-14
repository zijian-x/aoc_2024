use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn parser(file: &File) -> (Vec<i32>, Vec<i32>) {
    let mut data: (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        };

        for (i, elem) in line.split_whitespace().into_iter().enumerate() {
            if i == 0 {
                data.0.push(elem.parse().unwrap());
            } else {
                data.1.push(elem.parse().unwrap());
            }
        }

        line.clear();
    }

    data
}

pub fn part1(file: &File) {
    let mut data = parser(file);
    data.0.sort();
    data.1.sort();

    let mut sum = 0;
    for i in 0..data.0.len().min(data.1.len()) {
        sum += (data.0[i] - data.1[i]).abs();
    }

    println!("{sum}");
}

pub fn part2(file: &File) {
    let data = parser(file);
    let mut map = HashMap::<i32, i32>::new();

    data.1.iter().for_each(|elem| {
        *map.entry(*elem).or_insert(0) += 1;
    });

    let sum = data
        .0
        .iter()
        .fold(0, |acc, x| acc + *x * *map.entry(*x).or_default());
    println!("{}", sum);
}
