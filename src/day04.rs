use aoc_runner_derive::aoc;

// No generator today because lifetimes. See https://github.com/gobanos/cargo-aoc/issues/20
fn parse(input: &str) -> Vec<&[u8]> {
    input.as_bytes().split(|b| *b == b'\n').collect::<Vec<_>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u16 {
    let m = parse(input);

    let mut res = 0;

    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == b'X' {
                res += count_xmas(&m, r, c)
            }
        }
    }

    res
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u16 {
    let m = parse(input);

    let mut res = 0;

    // 'A' must be inside the border in valid arrangements, so we can cut off a couple of
    // iterations in both dimensions
    for r in 1..m.len() - 1 {
        for c in 1..m[0].len() - 1 {
            if m[r][c] == b'A' {
                res += is_x_mas(&m, r, c) as u16
            }
        }
    }

    res
}

fn get(m: &[&[u8]], r: usize, c: usize) -> u8 {
    *m.get(r).and_then(|row| row.get(c)).unwrap_or(&b'_')
}

/// Returns the count, given we have an X, of the surrounding squares of many MAS sequences we have hanging off that in every direction.
fn count_xmas(m: &[&[u8]], r: usize, c: usize) -> u16 {
    [
        (0_i16, -1_i16),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
    .iter()
    .filter(|(dr, dc)| {
        (1..4).all(|i| {
            let (rr, cc) = (r as i16 + (dr * i), c as i16 + (dc * i));
            if rr < 0 || cc < 0 || rr as usize >= m.len() || cc as usize >= m[0].len() {
                return false;
            }
            get(m, rr as usize, cc as usize) == b"XMAS"[i as usize]
        })
    })
    .count() as u16
}

/// Returns true if, given we have an A, search the surrounding corners of the immediate square to check that we have an MS or SM across the diagonals.
fn is_x_mas(m: &[&[u8]], r: usize, c: usize) -> bool {
    let w1 = [get(m, r - 1, c - 1), get(m, r + 1, c + 1)];
    let w2 = [get(m, r - 1, c + 1), get(m, r + 1, c - 1)];
    [w1, w2].iter().all(|w| w == b"MS" || w == b"SM")
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 9);
    }
}
