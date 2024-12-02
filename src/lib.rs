#[path = "day01.rs"]
pub mod day1;
#[path = "day02.rs"]
pub mod day2;

// POWERS_OF_10 is used in tandem with parse_number to try to take advantage of the values in our problem.
const POWERS_OF_10: [usize; 5] = [1, 10, 100, 1000, 10000];

fn parse_number(val: &str) -> usize {
    let radix = val.len() - 1;

    val.bytes()
        .enumerate()
        .map(|(n, c)| (c - 48) as usize * POWERS_OF_10[radix - n])
        .sum()
}

aoc_runner_derive::aoc_lib! { year = 2024 }
