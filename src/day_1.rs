use std::fs;

fn get_input() -> String {
    fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_1\\input.txt")
        .expect("Should be able to read the file at this location")
        .trim()
        .to_owned()
}

pub fn part_1() {
    let input = get_input();
}
