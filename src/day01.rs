use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    input.lines().for_each(|line| {
        // Just doing val.parse().unwrap() would be simpler.
        // But here we play fast and loose with number parsing.
        // We know our inputs are all positive. So we skip error handling, sign handling and other stuff this way.
        let mut parts = line.split_ascii_whitespace().map(parse_number);
        left.push(parts.next().unwrap());
        right.push(parts.next().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
}

// POWERS_OF_10 is used in tandem with parse_number to try to take advantage of the values in our problem.
const POWERS_OF_10: [usize; 5] = [1, 10, 100, 1000, 10000];

fn parse_number(val: &str) -> usize {
    let radix = val.len() - 1;

    val.bytes()
        .enumerate()
        .map(|(n, c)| (c - 48) as usize * POWERS_OF_10[radix - n])
        .sum()
}

#[aoc(day1, part1)]
fn part1_impl(sides: &(Vec<usize>, Vec<usize>)) -> usize {
    let (left, right) = sides;
    left.iter().zip(right).map(|(l, r)| l.abs_diff(*r)).sum()
}

#[aoc(day1, part2)]
fn part2_impl(sides: &(Vec<usize>, Vec<usize>)) -> usize {
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
                sum += x[0] * x.len() * y.len();
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
pub fn part1(input: &str) -> usize {
    part1_impl(&parse(input))
}

// For CodSpeed - see https://codspeed.io/advent
pub fn part2(input: &str) -> usize {
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
