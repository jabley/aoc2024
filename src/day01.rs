use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut parts = line
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap());
        left.push(parts.next().unwrap());
        right.push(parts.next().unwrap());
    });

    left.sort_unstable();
    right.sort_unstable();

    (left, right)
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
    let mut a = left.chunk_by(|x, y| x == y).peekable();
    let mut b = right.chunk_by(|x, y| x == y).peekable();

    let mut sum = 0;

    loop {
        let Some(&x) = a.peek() else {
            break;
        };
        let Some(&y) = b.peek() else {
            break;
        };
        match x[0].cmp(&y[0]) {
            std::cmp::Ordering::Less => {
                a.next();
            }
            std::cmp::Ordering::Equal => {
                sum += x[0] * x.len() * y.len();
                a.next();
                b.next();
            }
            std::cmp::Ordering::Greater => {
                b.next();
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
}
