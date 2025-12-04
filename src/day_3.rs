use std::fs;

fn get_input() -> Vec<String> {
    fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_3\\input.txt")
        .expect("Should be able to read the file at this location")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn max_joltage(bank: &str) -> u32 {
    let chars = bank.chars();
    let digits = chars.map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let max_digit = *digits.iter().max().unwrap();
    for i in 0..max_digit {
        let digit = max_digit - i;
        let idx_opt = digits.iter().position(|&n| n == digit);
        if let Some(idx) = idx_opt {
            if idx == digits.len() - 1 {
                // This digit is at the end, so we can't make a joltage with it
                continue;
            }
            let next_digit = *digits[(idx + 1)..].iter().max().unwrap();
            return 10 * digit + next_digit;
        } else {
            continue;
        }
    }
    unreachable!("Should have exited the loop at some point");
}

fn total_max_joltage(banks: &[String]) -> u32 {
    banks.iter().map(|b| max_joltage(b)).sum()
}

pub fn part_1() {
    let banks = get_input();
    println!("The maximum possible total joltage is {}", total_max_joltage(&banks));
}
