use aoc_runner_derive::{aoc, aoc_generator};

use crate::parse_number;

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    input.lines().for_each(|line| {
        // Just doing val.parse().unwrap() would be simpler.
        // But here we play fast and loose with number parsing.
        // We know our inputs are all positive. So we skip error handling, sign handling and other stuff this way.
        let mut parts = line.split_ascii_whitespace().map(parse_number);
        left.push(unsafe { parts.next().unwrap_unchecked() });
        right.push(unsafe { parts.next().unwrap_unchecked() });
    });

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

#[aoc(day1, part1)]
fn part1_impl(sides: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = sides;
    left.iter().zip(right).map(|(l, r)| l.abs_diff(*r)).sum()
}

#[aoc(day1, part2)]
fn part2_impl(sides: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (left, right) = sides;

    // We have sorted the lists as part of parsing. Chunk by the distinct values in each list and
    // make it peekable because there aren't any side-effects here to be concerned about.
    let mut left = left.chunk_by(|x, y| x == y).peekable();
    let mut right = right.chunk_by(|x, y| x == y).peekable();

    let mut sum = 0;

    loop {
        let Some(&x) = left.peek() else {
            break;
        };
        let Some(&y) = right.peek() else {
            break;
        };
        match x[0].cmp(&y[0]) {
            std::cmp::Ordering::Less => {
                left.next();
            }
            std::cmp::Ordering::Equal => {
                sum += x[0] * x.len() as u32 * y.len() as u32;
                left.next();
                right.next();
            }
            std::cmp::Ordering::Greater => {
                right.next();
            }
        }
    }
    sum
}

// For CodSpeed - see https://codspeed.io/advent
pub fn part1(input: &str) -> u32 {
    part1_impl(&parse(input))
}

// For CodSpeed - see https://codspeed.io/advent
pub fn part2(input: &str) -> u32 {
    part2_impl(&parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1_impl(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            11
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_impl(&parse(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            31
        );
    }

    #[test]
    fn hand_parsing() {
        assert_eq!(91527, parse_number("91527"))
    }
}
