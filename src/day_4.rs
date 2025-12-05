use std::collections::HashSet;
use std::fs;

fn _get_input() -> HashSet<(isize, isize)> {
    let mut input = HashSet::new();

    for (row, line) in fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_4\\input.txt")
        .expect("Should be able to read the file at this location")
        .lines()
        .enumerate()
    {
        for (col, char) in line.chars().enumerate() {
            if char == '@' {
                input.insert((row as isize, col as isize));
            }
        }
    }

    input
}

fn _adjacent_positions(row: isize, col: isize) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if !(row_offset == 0 && col_offset == 0) {
                result.push((row + row_offset, col + col_offset));
            }
        }
    }
    assert_eq!(result.len(), 8);
    result
}

fn _is_accessible(paper: &HashSet<(isize, isize)>, row: isize, col: isize) -> bool {
    _adjacent_positions(row, col).iter()
        .filter(|coords| paper.contains(&(coords.0, coords.1)))
        .count() < 4
}

fn _count_accessible(paper: &HashSet<(isize, isize)>) -> usize {
    paper.iter().filter(|coords| _is_accessible(paper, coords.0, coords.1)).count()
}

fn _remove_accessible(paper: &mut HashSet<(isize, isize)>) -> usize {
    let mut to_remove = Vec::new();
    for coords in paper.iter() {
        if _is_accessible(paper, coords.0, coords.1) {
            to_remove.push(*coords);
        }
    }
    for coords in to_remove.iter() {
        paper.remove(&coords);
    }
    to_remove.len()
}

fn _remove_as_much_as_possible(paper: &mut HashSet<(isize, isize)>) -> usize {
    let mut removed = 0;
    loop {
        let removed_this_loop = _remove_accessible(paper);
        removed += removed_this_loop;
        if removed_this_loop == 0 {
            break;
        }
    }
    removed
}

pub fn _part_1() {
    let paper = _get_input();
    println!("{} rolls are accessible", _count_accessible(&paper));
}

pub fn _part_2() {
    let mut paper = _get_input();
    println!("{} rolls can be removed", _remove_as_much_as_possible(&mut paper));
}
