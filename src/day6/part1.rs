use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

impl Direction {
    fn advance(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let term = match self {
            North | South => &pos.0,
            West | East => &pos.1,
        };
        let (advanced, overflowed) = match self {
            North | West => term.overflowing_sub(1),
            South | East => term.overflowing_add(1),
        };

        if overflowed {
            return None;
        }
        match self {
            North | South => Some((advanced, pos.1)),
            West | East => Some((pos.0, advanced)),
        }
    }

    fn advance_until<F>(
        &self,
        mut guard: (usize, usize),
        set: &mut HashSet<(usize, usize)>,
        limit_reached: F,
    ) -> Option<((usize, usize), usize)>
    where
        F: Fn((usize, usize)) -> bool,
    {
        let mut cnt = 0;
        while let Some(next_pos) = self.advance(guard) {
            if limit_reached(next_pos) {
                return Some((guard, cnt));
            }
            guard = next_pos;
            cnt += set.insert(guard) as usize;
        }

        None
    }

    fn turn_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

fn parse(file: &File) -> Vec<Vec<u8>> {
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.into_bytes())
        .collect::<Vec<Vec<u8>>>()
}

fn find_guard(mtx: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for (i, row) in mtx.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == b'^' {
                return Some((i, j));
            }
        }
    }

    None
}

fn find_next_obstruction(
    mtx: &Vec<Vec<u8>>,
    mut guard_pos: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    while let Some((next_x, next_y)) = direction.advance(guard_pos) {
        if next_x >= mtx.len() || next_y >= mtx[next_x].len() {
            return None;
        }

        if mtx[next_x][next_y] == b'#' {
            return Some((next_x, next_y));
        }
        guard_pos = (next_x, next_y);
    }
    None
}

pub fn run(file: File) {
    let mtx = parse(&file);

    let mut guard = find_guard(&mtx).unwrap();
    let mut direction = North;

    let mut cnt = 1usize;
    let mut set = HashSet::new();
    set.insert(guard);

    while let Some(obs) = find_next_obstruction(&mtx, guard, direction) {
        let (new_pos, steps) = direction
            .advance_until(guard, &mut set, |guard| guard == obs)
            .unwrap();
        guard = new_pos;
        cnt += steps;

        direction = direction.turn_right();
    }

    while let Some(next_pos) = direction.advance(guard) {
        if next_pos.0 >= mtx.len() || next_pos.1 >= mtx[0].len() {
            break;
        }
        guard = next_pos;
        cnt += set.insert(guard) as usize;
    }

    println!("{}", cnt);
}

#[test]
fn test_advanced() {
    let idx = (1, 1);

    assert_eq!(North.advance(idx), Some((0, 1)));
    assert_eq!(South.advance(idx), Some((2, 1)));
    assert_eq!(West.advance(idx), Some((1, 0)));
    assert_eq!(East.advance(idx), Some((1, 2)));

    let idx = (0, 0);
    assert_eq!(North.advance(idx), None);
    assert_eq!(South.advance(idx), Some((1, 0)));
    assert_eq!(West.advance(idx), None);
    assert_eq!(East.advance(idx), Some((0, 1)));
}
