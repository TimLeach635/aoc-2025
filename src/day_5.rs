use std::fs;
use regex::Regex;

fn get_input() -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();

    let range_re = Regex::new(r"(?<start>\d+)-(?<end>\d+)").unwrap();

    for line in fs::read_to_string("C:\\NotWork\\advent-of-code\\aoc-2025\\src\\day_5\\input.txt")
        .expect("Should be able to read the file at this location")
        .lines()
    {
        if let Some(caps) = range_re.captures(line) {
            ranges.push((
                caps["start"].parse().unwrap(),
                caps["end"].parse().unwrap()
            ));
        } else if let Ok(id) = line.parse::<usize>() {
            ingredients.push(id);
        }
    }

    (ranges, ingredients)
}

fn lies_in_range(id: usize, start: usize, end: usize) -> bool {
    start <= id && id <= end
}

fn lies_in_any_range(id: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|&(start, end)| lies_in_range(id, start, end))
}

fn count_fresh(ids: &[usize], ranges: &[(usize, usize)]) -> usize {
    ids.iter().filter(|&&id| lies_in_any_range(id, &ranges)).count()
}

pub fn part_1() {
    let (ranges, ingredients) = get_input();
    println!("There are {} fresh ingredients", count_fresh(&ingredients, &ranges));
}
