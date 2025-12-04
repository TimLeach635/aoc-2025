use std::fs;
use fancy_regex::Regex;

fn get_input() -> Vec<(usize, usize)> {
    let re = Regex::new(r"(?<first>\d+)-(?<last>\d+)")
        .expect("Should be able to create regex");
    fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_2\\input.txt")
        .expect("Should be able to read the file at this location")
        .split(",")
        .map(|s| {
            let caps = re.captures(s)
                .expect("All parts should match regex")
                .expect("Should have captures");
            (caps["first"].parse().unwrap(), caps["last"].parse().unwrap())
        })
        .collect()
}

fn is_valid(id: usize) -> bool {
    // Number of digits
    let n = id.ilog10() + 1;
    let n_2 = n / 2;
    for i in 1..=n_2 {
        // Does this sequence length evenly divide?
        if n % i != 0 {
            continue;
        }
        let t = 10u32.pow(i);
        let seq = id % (t as usize);
        let mut rep = seq;
        for _ in 1..(n / i) {
            rep *= t as usize;
            rep += seq;
        }
        if rep == id {
            return false
        }
    }
    true
}

fn invalid_ids_in_range(first: usize, last: usize) -> Vec<usize> {
    (first..=last).filter(|i| !is_valid(*i)).collect()
}

fn invalid_ids_in_ranges(ranges: &[(usize, usize)]) -> Vec<usize> {
    ranges.iter()
        .flat_map(|r| invalid_ids_in_range(r.0, r.1))
        .collect()
}

pub fn part_2() {
    let input = get_input();
    println!("The sum of the invalid ids is {}", invalid_ids_in_ranges(&input).iter().sum::<usize>());
}
