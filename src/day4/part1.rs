use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(file: &File) -> Vec<Vec<u8>> {
    BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(String::into_bytes)
        .collect()
}

pub fn run(file: &File) {
    let mtx = parse(file);
    let xmas = [b'X', b'M', b'A', b'S'];

    let check_horizontal = |mtx: &Vec<Vec<u8>>, i: usize, j: usize| -> usize {
        let mut cnt = 0;

        let (res, overflowed) = j.overflowing_add(3);
        if !overflowed && res < mtx[i].len() {
            let k = (0..4).take_while(|x| mtx[i][j + x] == xmas[*x]).count();
            cnt += (k == 4) as usize;
        }

        let (_, underflowed) = j.overflowing_sub(3);
        if !underflowed {
            let k = (0..4).take_while(|x| mtx[i][j - x] == xmas[*x]).count();
            cnt += (k == 4) as usize;
        }

        cnt
    };

    let check_vertical = |mtx: &Vec<Vec<u8>>, i: usize, j: usize| -> usize {
        let mut cnt = 0;

        let (res, overflowed) = i.overflowing_add(3);
        if !overflowed && res < mtx.len() {
            let k = (0..4).take_while(|x| mtx[i + x][j] == xmas[*x]).count();
            cnt += (k == 4) as usize;
        }

        let (_, underflowed) = i.overflowing_sub(3);
        if !underflowed {
            let k = (0..4).take_while(|x| mtx[i - x][j] == xmas[*x]).count();
            cnt += (k == 4) as usize;
        }

        cnt
    };

    let check_diagonal = |mtx: &Vec<Vec<u8>>, i: usize, j: usize| -> usize {
        let mut cnt = 0;

        let (i_, overflowed_i) = i.overflowing_add(3);
        let (j_, overflowed_j) = j.overflowing_add(3);
        if !overflowed_i && !overflowed_j && i_ < mtx.len() && j_ < mtx[i_].len() {
            let k = (0..=3)
                .take_while(|k| mtx[i + k][j + k] == xmas[*k])
                .count();
            cnt += (k == 4) as usize;
        }

        let (_, underflowed_i) = i.overflowing_sub(3);
        let (_, underflowed_j) = j.overflowing_sub(3);
        if !underflowed_i && !underflowed_j {
            let k = (0..=3)
                .take_while(|k| mtx[i - k][j - k] == xmas[*k])
                .count();
            cnt += (k == 4) as usize;
        }

        let (res_i, overflowed_i) = i.overflowing_add(3);
        let (_, underflowed_j) = j.overflowing_sub(3);
        if !overflowed_i && res_i < mtx.len() && !underflowed_j {
            let k = (0..=3)
                .take_while(|k| mtx[i + k][j - k] == xmas[*k])
                .count();
            cnt += (k == 4) as usize;
        }


        let (_, underflowed_i) = i.overflowing_sub(3);
        let (res_j, overflowed_j) = j.overflowing_add(3);
        if !underflowed_i && !overflowed_j && res_j < mtx[0].len() {
            let k = (0..=3)
                .take_while(|k| mtx[i - k][j + k] == xmas[*k])
                .count();
            cnt += (k == 4) as usize;
        }
        cnt
    };

    let mut sum = 0;
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            sum += check_horizontal(&mtx, i, j)
                + check_vertical(&mtx, i, j)
                + check_diagonal(&mtx, i, j);
        }
    }

    println!("{}", sum);
}
