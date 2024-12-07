use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day6)]
fn parse(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    // my puzzle input is 130*130, presumably to prevent people from using u128 as a compact representation of each line.
    // you only need to represent the grid with obstacle locations and highlight the starting point.
    let mut start = (0, 0);

    // bool and u8 both are size 1, align 0x1, so no real benefit to converting to a Vec of Vec of bools though.
    let mut found_start = false;

    (
        input
            .as_bytes()
            .split(|b| *b == b'\n')
            .enumerate()
            .map(|(r, l)| {
                if found_start {
                    l.to_vec()
                } else {
                    let mut res = Vec::with_capacity(l.len());

                    for (c, cell) in l.iter().enumerate() {
                        match cell {
                            b'^' => {
                                start = (r, c);
                                found_start = true;
                            }
                            _ => {}
                        }
                        res.push(*cell)
                    }
                    res
                }
            })
            .collect::<Vec<_>>(),
        start,
    )
}

#[aoc(day6, part1)]
fn part1_impl(input: &(Vec<Vec<u8>>, (usize, usize))) -> usize {
    let (map, start) = input;

    patrol(&map, start.0, start.1, true).unwrap().len()
}

#[aoc(day6, part2)]
fn part2_impl(input: &(Vec<Vec<u8>>, (usize, usize))) -> usize {
    let (map, start) = input;

    // FIXME: this seems a bit rubbish?
    let mut map = map.clone();

    let path = patrol(&map, start.0, start.1, true).unwrap();

    // Walk the path from part 1, turning every cell on the path into an obstacle and seeing if we
    // get a loop.
    // This is a lazy brute force approach. I thought I would be able to optimise it by looking at
    // potential corners, but not got that working yet.
    path.iter()
        .filter(|&&(r, c)| {
            map[r][c] = b'#';
            let ok = patrol(&map, start.0, start.1, false).is_none();
            map[r][c] = b'.';
            ok
        })
        .count()
}

fn patrol(
    m: &[Vec<u8>],
    mut r: usize,
    mut c: usize,
    return_squares: bool,
) -> Option<Vec<(usize, usize)>> {
    let mut seen = vec![vec![[false; 4]; m[0].len()]; m.len()];
    let mut d = 0;

    loop {
        if seen[r][c][d] {
            return None;
        }

        seen[r][c][d] = true;
        let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][d];
        let (rr, cc) = (r.wrapping_add(dr as usize), c.wrapping_add(dc as usize));

        if !(0..m.len()).contains(&rr) || !(0..m[0].len()).contains(&cc) {
            if !return_squares {
                return Some(Vec::new());
            }
            let visited = (0..m.len())
                .cartesian_product(0..m[0].len())
                .filter(|&(r, c)| seen[r][c].iter().any(|&b| b))
                .collect();
            return Some(visited);
        }

        if m[rr][cc] == b'#' {
            d = (d + 1) % 4;
        } else {
            (r, c) = (rr, cc);
        }
    }
}

pub fn part1(input: &str) -> usize {
    part1_impl(&parse(input))
}

pub fn part2(input: &str) -> usize {
    part2_impl(&parse(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&INPUT), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&INPUT), 6);
    }
}
