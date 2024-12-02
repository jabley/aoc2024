use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<u16>> {
    let mut res = Vec::with_capacity(1000);

    input.as_bytes().split(|b| *b == b'\n').for_each(|l| {
        let mut levels = Vec::with_capacity(8);

        for level in l
            .split(|b| *b == b' ')
            // skip empty lines like the last one in the input file...
            .filter(|v| !v.is_empty())
            .map(parse_u16)
        {
            levels.push(level);
        }

        res.push(levels);
    });

    res
}

fn parse_u16(v: &[u8]) -> u16 {
    // In our problem input, we only have 1 or 2 digit numbers. Optimise for that.
    match v.len() {
        // b'8' => 8_u16
        1 => unsafe { (v.get_unchecked(0) - 48) as u16 },
        // b'85' => (8 * 10 + 5)_u16
        2 => unsafe { (v.get_unchecked(0) - 48) as u16 * 10 + (v.get_unchecked(1) - 48) as u16 },
        _ => panic!("Not parsing '{:?}' into a u16", v),
    }
}

#[aoc(day2, part1)]
fn part1_impl(input: &[Vec<u16>]) -> u16 {
    input
        .iter()
        .map(|report| match is_safe(report) {
            Some(_) => 0,
            None => 1,
        })
        .sum()
}

fn is_safe(report: &[u16]) -> Option<usize> {
    let mut direction: Option<std::cmp::Ordering> = None;

    for (i, pair) in report.windows(2).enumerate() {
        let diff = pair[0].abs_diff(pair[1]);

        if !(1..=3).contains(&diff) {
            return Some(i);
        }

        match pair[0].cmp(&pair[1]) {
            std::cmp::Ordering::Less => match direction {
                None => {
                    direction = Some(std::cmp::Ordering::Less);
                }
                Some(std::cmp::Ordering::Less) => {}
                _ => return Some(i),
            },
            std::cmp::Ordering::Equal => return Some(i),
            std::cmp::Ordering::Greater => match direction {
                Some(std::cmp::Ordering::Greater) => {}
                None => direction = Some(std::cmp::Ordering::Greater),
                _ => return Some(i),
            },
        }
    }
    None
}

#[aoc(day2, part2)]
fn part2_impl(input: &[Vec<u16>]) -> u16 {
    input
        .iter()
        .map(|report| match is_safe(report) {
            None => 1,
            Some(i) => {
                if i > 0 && i < report.len() {
                    let mut skip_previous_input = report.clone();
                    skip_previous_input.remove(i - 1);
                    // let skip_previous_input = &[&report[..(i - 1)], &report[i..]].concat();
                    if is_safe(&skip_previous_input).is_none() {
                        return 1;
                    }
                }

                let mut skip_input = report.clone();
                skip_input.remove(i);
                // let skip_input = &[&report[..i], &report[(i + 1)..]].concat();
                if is_safe(&skip_input).is_none() {
                    1
                } else {
                    let skip_next = if i < report.len() - 1 {
                        let mut skip_next_input = report.clone();
                        skip_next_input.remove(i + 1);
                        // let skip_next_level = &[&report[..(i + 1)], &report[(i + 2)..]].concat();
                        is_safe(&skip_next_input)
                    } else {
                        Some(i)
                    };
                    if skip_next.is_none() {
                        1
                    } else {
                        0
                    }
                }
            }
        })
        .sum()
}

pub fn part1(input: &str) -> u16 {
    part1_impl(&parse(input))
}

pub fn part2(input: &str) -> u16 {
    part2_impl(&parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1_impl(&parse(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            2
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_impl(&parse(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            )),
            4
        );
    }

    #[test]
    fn part2_boundary_conditions() {
        assert_eq!(
            part2_impl(&parse("1 3 6 7 9 5")),
            1,
            "last level can be dampened"
        );
        assert_eq!(
            part2_impl(&parse("9 1 2 3 4 5")),
            1,
            "first level can be dampened"
        );
        assert_eq!(
            part2_impl(&parse("5 2 3 4 5")),
            1,
            "first level can be dampened"
        );
        assert_eq!(
            part2_impl(&parse("1 2 2 4 7")),
            1,
            "second level can be dampened"
        );
    }
}
