use array2d::Array2D;
use std::fs;

enum Operation {
    Add,
    Multiply,
}

struct Problem {
    n_1: usize,
    n_2: usize,
    n_3: usize,
    n_4: usize,
    operation: Operation,
}

fn get_input() -> Vec<Problem> {
    let array: Array2D<String> = Array2D::from_rows(
        &fs::read_to_string("/Users/tl673/NotWork/aoc-2025/src/day_6/input.txt")
            .expect("Should be able to read the file at this location")
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>(),
    )
    .unwrap();

    array
        .columns_iter()
        .map(|col| {
            let values: Vec<&String> = col.collect();

            Problem {
                n_1: values[0].parse().unwrap(),
                n_2: values[1].parse().unwrap(),
                n_3: values[2].parse().unwrap(),
                n_4: values[3].parse().unwrap(),
                operation: match values[4].as_str() {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => panic!("Unknown operation \"{}\"", values[4]),
                },
            }
        })
        .collect()
}

fn calculate(problem: &Problem) -> usize {
    match problem.operation {
        Operation::Add => problem.n_1 + problem.n_2 + problem.n_3 + problem.n_4,
        Operation::Multiply => problem.n_1 * problem.n_2 * problem.n_3 * problem.n_4,
    }
}

pub fn part_1() {
    let problems = get_input();
    println!(
        "The sum of all the answers is {}",
        problems.iter().map(|p| calculate(p)).sum::<usize>()
    );
}
