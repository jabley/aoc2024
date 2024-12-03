use aoc_runner_derive::{aoc, aoc_generator};
use regex::bytes::Regex;

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1_impl(input: &str) -> u32 {
    let rx = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    rx.find_iter(input.as_bytes())
        .map(|matches| eval_mul_expression(matches.as_bytes()))
        .sum()
}

/// Given an expression `mul(number1,number2)`, evaluate it.
fn eval_mul_expression(expression: &[u8]) -> u32 {
    // skip the b"mul(" and trailing b")", then split into 2 numeric strings
    expression[4..expression.len() - 1]
        .split(|b| *b == b',')
        .map(parse_u16)
        .reduce(|acc, val| val * acc)
        .unwrap() as u32
    // let first = parse_u16(parts.next().unwrap());
    // let second = parse_u16(parts.next().unwrap());
    // (first * second) as u32
}

fn parse_u16(v: &[u8]) -> u16 {
    // In our problem input, we only have 1, 2, or 3 digit numbers. Optimise for that.
    match v.len() {
        // b'8' => 8_u16
        1 => unsafe { (v.get_unchecked(0) - 48) as u16 },
        // b'85' => (8 * 10 + 5)_u16
        2 => unsafe { (v.get_unchecked(0) - 48) as u16 * 10 + (v.get_unchecked(1) - 48) as u16 },
        // b'785' => (7 * 100 + 8 * 10 + 5)_u16
        3 => unsafe {
            (v.get_unchecked(0) - b'0') as u16 * 100
                + (v.get_unchecked(1) - b'0') as u16 * 10
                + (v.get_unchecked(2) - b'0') as u16
        },
        _ => panic!("Not parsing '{:?}' into a u16", v),
    }
}

#[aoc(day3, part2)]
fn part2_impl(input: &str) -> u32 {
    let rx = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    rx.find_iter(input.as_bytes())
        .map(|rx_match| rx_match.as_bytes())
        .fold((0, true), |(acc, enabled), instruction| match instruction {
            b"do()" => (acc, true),
            b"don't()" => (acc, false),
            _ => {
                if enabled {
                    (acc + eval_mul_expression(instruction), true)
                } else {
                    (acc, enabled)
                }
            }
        })
        .0
}

pub fn part1(input: &str) -> u32 {
    part1_impl(&parse(input))
}

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
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_impl(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
