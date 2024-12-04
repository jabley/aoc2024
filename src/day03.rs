use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    ascii::dec_uint,
    combinator::{delimited, separated_pair},
    PResult, Parser,
};

#[derive(Eq, PartialEq, Debug)]
struct Mul((u32, u32));

impl Mul {
    fn eval(&self) -> u32 {
        self.0 .0 * self.0 .1
    }
}

/// Parses something like 123,456
fn number_pair(i: &mut &[u8]) -> PResult<(u32, u32)> {
    separated_pair(dec_uint, ',', dec_uint).parse_next(i)
}

/// Parses a mul instruction like `mul(123, 44)`
fn mul(i: &mut &[u8]) -> PResult<Mul> {
    delimited("mul(", number_pair, ')').map(Mul).parse_next(i)
}

#[aoc_generator(day3, part1)]
fn parse_part1(input: &str) -> Vec<Mul> {
    let mut res = Vec::with_capacity(8);

    let mut i = 0;

    let input = input.as_bytes();

    while i < input.len() {
        let mut suffix = &input[i..];
        let n = suffix.len();

        if let Ok(mul) = mul(&mut suffix) {
            res.push(mul);
            let chars_parsed = n - suffix.len();
            i += chars_parsed;
            continue;
        }

        i += 1;
    }

    res
}

#[aoc(day3, part1)]
fn part1_impl(input: &[Mul]) -> u32 {
    input.iter().map(|m| m.eval()).sum()
}

#[aoc_generator(day3, part2)]
fn parse_part2(input: &str) -> Vec<Mul> {
    let mut res = Vec::with_capacity(8);

    let mut i = 0;

    let mut enabled = true;

    let input = input.as_bytes();

    while i < input.len() {
        if input[i..].starts_with(b"do()") {
            enabled = true;
            i += 4;
            continue;
        }
        if input[i..].starts_with(b"don't()") {
            enabled = false;
            i += 7;
            continue;
        }

        let mut suffix = &input[i..];
        let n = suffix.len();

        if enabled {
            if let Ok(mul) = mul(&mut suffix) {
                res.push(mul);
                let chars_parsed = n - suffix.len();
                i += chars_parsed;
                continue;
            }
        }

        i += 1;
    }

    res
}

#[aoc(day3, part2)]
fn part2_impl(input: &[Mul]) -> u32 {
    input.iter().map(|m| m.eval()).sum()
}

pub fn part1(input: &str) -> u32 {
    part1_impl(&parse_part1(input))
}

pub fn part2(input: &str) -> u32 {
    part2_impl(&parse_part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
