use std::fs;

fn get_input() -> Vec<String> {
    fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_3\\input.txt")
        .expect("Should be able to read the file at this location")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn max_joltage(bank: &str, n_digits: usize) -> usize {
    let chars = bank.chars();
    let digits = chars
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut max_digit = *digits.iter().max().unwrap();
    if n_digits == 1 {
        return max_digit;
    }
    loop {
        let pos_opt = digits.iter().position(|&n| n == max_digit);
        if let Some(pos) = pos_opt {
            // Are there enough digits after this point to form a number?
            if digits.len() - pos >= n_digits {
                // Yes, there are
                let result = (max_digit * 10usize.pow(n_digits as u32 - 1))
                    + max_joltage(&bank[(pos + 1)..], n_digits - 1);
                return result
            }
        }
        max_digit -= 1;
    }
}

fn total_max_joltage(banks: &[String], n_digits: usize) -> usize {
    banks.iter().map(|b| max_joltage(b, n_digits)).sum()
}

pub fn _part_1() {
    let banks = get_input();
    println!("The maximum possible total joltage is {}", total_max_joltage(&banks, 2));
}

pub fn part_2() {
    let banks = get_input();
    println!("The maximum possible total joltage is {}", total_max_joltage(&banks, 12));
}
