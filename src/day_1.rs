use std::fs;
use regex::Regex;

fn get_input() -> Vec<isize> {
    let re = Regex::new(r"(?<direction>[LR])(?<amount>\d+)")
        .expect("Should be able to create regex");
    fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_1\\input.txt")
        .expect("Should be able to read the file at this location")
        .lines()
        .map(|l| {
            let caps = re.captures(l).expect("All lines should match regex");
            match &caps["direction"] {
                "L" => -caps["amount"].parse::<isize>().expect("Should be able to parse"),
                "R" => caps["amount"].parse::<isize>().expect("Should be able to parse"),
                _ => panic!("Unknown direction {}", &caps["direction"]),
            }
        })
        .collect()
}

fn apply_rotation(dial: &mut usize, rotation: isize) {
    let mut result: isize = *dial as isize + rotation;
    while result < 0 {
        result += 100;
    }
    while result > 99 {
        result -= 100;
    }
    *dial = result.try_into().expect("Should be able to convert to usize");
}

fn how_many_zeroes(rotations: &[isize]) -> usize {
    let mut result: usize = 0;
    let mut dial: usize = 50;
    for rotation in rotations {
        apply_rotation(&mut dial, *rotation);
        if dial == 0 {
            result += 1;
        }
    }
    result
}

pub fn part_1() {
    let input = get_input();
    println!("Lands on zero {} times", how_many_zeroes(&input));
}
