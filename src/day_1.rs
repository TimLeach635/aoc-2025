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

fn click_left(dial: &mut usize, zero_count: &mut usize) {
    if *dial > 0 {
        *dial -= 1;
        if *dial == 0 {
            *zero_count += 1;
        }
    } else {
        *dial = 99;
    }
}

fn click_right(dial: &mut usize, zero_count: &mut usize) {
    if *dial < 99 {
        *dial += 1;
    } else {
        *dial = 0;
        *zero_count += 1;
    }
}

fn apply_rotation(dial: &mut usize, rotation: isize, zero_count: &mut usize) {
    if rotation > 0 {
        for _ in 0..rotation {
            click_right(dial, zero_count);
        }
    } else {
        for _ in 0..-rotation {
            click_left(dial, zero_count);
        }
    }
}

fn _how_many_zeroes(rotations: &[isize]) -> usize {
    let mut result: usize = 0;
    let mut _zero_count: usize = 0;
    let mut dial: usize = 50;
    for rotation in rotations {
        apply_rotation(&mut dial, *rotation, &mut _zero_count);
        if dial == 0 {
            result += 1;
        }
    }
    result
}

fn how_many_zero_clicks(rotations: &[isize]) -> usize {
    let mut zero_count: usize = 0;
    let mut dial: usize = 50;
    for rotation in rotations {
        apply_rotation(&mut dial, *rotation, &mut zero_count);
    }
    zero_count
}

pub fn _part_1() {
    let input = get_input();
    println!("Lands on zero {} times", _how_many_zeroes(&input));
}

pub fn part_2() {
    let input = get_input();
    println!("Clicks past zero {} times", how_many_zero_clicks(&input));
}
