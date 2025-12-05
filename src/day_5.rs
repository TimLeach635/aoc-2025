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

fn _lies_in_any_range(id: usize, ranges: &[(usize, usize)]) -> bool {
    ranges.iter().any(|&(start, end)| lies_in_range(id, start, end))
}

fn _count_fresh(ids: &[usize], ranges: &[(usize, usize)]) -> usize {
    ids.iter().filter(|&&id| _lies_in_any_range(id, &ranges)).count()
}

pub fn _part_1() {
    let (ranges, ingredients) = get_input();
    println!("There are {} fresh ingredients", _count_fresh(&ingredients, &ranges));
}

fn combine_ranges(ranges: &mut Vec<(usize, usize)>, (new_start, new_end): (usize, usize)) {
    // We can remove any ranges that lie entirely within this new range
    let mut current_idx = 0;
    while current_idx < ranges.len() {
        let (start, end) = ranges[current_idx];
        if lies_in_range(start, new_start, new_end) && lies_in_range(end, new_start, new_end) {
            ranges.swap_remove(current_idx);
        } else {
            current_idx += 1;
        }
    }

    // There are some tricky edge cases here. First I want to check to see if this new range
    // joins any existing ranges together - this will happen if the endpoints lie in different
    // ranges.
    let range_containing_start_idx_opt = ranges.iter()
        .position(|&(start, end)| lies_in_range(new_start, start, end));
    let range_containing_end_idx_opt = ranges.iter()
        .position(|&(start, end)| lies_in_range(new_end, start, end));
    // These should be unique - any given number should only fall within a single range, otherwise
    // there's an overlap that we should have removed already

    // Cases:
    // 1. The new range lies entirely within a single existing range, and can be discarded
    // 2. The new range starts in one range, and ends in another. We need to join all three ranges
    // 3. The new range starts in an existing range, so that range should be extended
    // 4. The new range ends in an existing range, so that range should be extended
    // 5. The new range is completely independent of all existing ranges, so just gets added

    // Case 5
    if range_containing_start_idx_opt.is_none() && range_containing_end_idx_opt.is_none() {
        ranges.push((new_start, new_end));
        return;
    }

    if let Some(range_containing_start_idx) = range_containing_start_idx_opt {
        if let Some(range_containing_end_idx) = range_containing_end_idx_opt {
            if range_containing_start_idx == range_containing_end_idx {
                // Case 1
                return;
            } else {
                // Case 2
                if range_containing_start_idx < range_containing_end_idx {
                    let new_end = ranges[range_containing_end_idx].1;
                    ranges.swap_remove(range_containing_end_idx);
                    ranges[range_containing_start_idx].1 = new_end;
                } else {
                    let new_start = ranges[range_containing_start_idx].0;
                    ranges.swap_remove(range_containing_start_idx);
                    ranges[range_containing_end_idx].0 = new_start;
                }
            }
        } else {
            // Case 3
            ranges[range_containing_start_idx].1 = new_end;
        }
    } else {
        // Case 4
        let range_containing_end_idx = range_containing_end_idx_opt
            .expect("This case should have been ruled out by previous conditions");
        ranges[range_containing_end_idx].0 = new_start;
    }
}

fn remove_overlaps(ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for &new_range in ranges {
        combine_ranges(&mut result, new_range);
    }
    result
}

fn size((start, end): (usize, usize)) -> usize {
    end - start + 1
}

fn count_all_fresh(ranges: &[(usize, usize)]) -> usize {
    remove_overlaps(ranges).iter().map(|&range| size(range)).sum()
}

pub fn part_2() {
    let (ranges, _) = get_input();
    println!("There are {} possible fresh ingredients", count_all_fresh(&ranges));
}
