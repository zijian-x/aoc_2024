use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file: &File) -> (Vec<String>, Vec<String>) {
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .partition(|line| line.len() == 5)
}

pub fn run(file: File) {
    let (rules, page_rows) = parse(&file);

    let mut page_map: HashMap<i32, (HashSet<i32>, HashSet<i32>)> = HashMap::new();
    rules
        .into_iter()
        .map(|line| {
            line.split_once('|')
                .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
                .unwrap()
        })
        .for_each(|(n0, n1)| {
            page_map
                .entry(n0)
                .or_insert_with(|| (HashSet::new(), HashSet::new()))
                .1
                .insert(n1);
            page_map
                .entry(n1)
                .or_insert_with(|| (HashSet::new(), HashSet::new()))
                .0
                .insert(n0);
        });

    let page_rows = page_rows
        .iter()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for pages in page_rows {
        let mut cnt = 0;
        for (j, page) in pages.iter().enumerate() {
            let cnt_before = pages[0..j]
                .iter()
                .filter(|p| page_map.get(&page).unwrap().0.contains(p))
                .count();
            let cnt_after = pages[j + 1..]
                .iter()
                .filter(|p| page_map.get(&page).unwrap().1.contains(p))
                .count();

            if cnt_before == j && cnt_after == pages.len() - j - 1 {
                cnt += 1;
            }
        }

        if cnt == pages.len() {
            sum += pages[pages.len() / 2];
        }
    }

    println!("{}", sum);
}
