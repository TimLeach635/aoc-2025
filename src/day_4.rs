use std::collections::HashSet;
use std::fs;

fn get_input() -> HashSet<(isize, isize)> {
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

fn adjacent_positions(row: isize, col: isize) -> Vec<(isize, isize)> {
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

fn is_accessible(paper: &HashSet<(isize, isize)>, row: isize, col: isize) -> bool {
    adjacent_positions(row, col).iter()
        .filter(|coords| paper.contains(&(coords.0, coords.1)))
        .count() < 4
}

fn count_accessible(paper: &HashSet<(isize, isize)>) -> usize {
    paper.iter().filter(|coords| is_accessible(paper, coords.0, coords.1)).count()
}

pub fn part_1() {
    let paper = get_input();
    println!("{} rolls are accessible", count_accessible(&paper));
}
