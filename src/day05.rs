use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use crate::parse_number;

#[aoc_generator(day5)]
fn parse(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::<u32, HashSet<u32>>::default();

    for l in s1.lines() {
        let (x, y) = l.split_once('|').unwrap();
        rules
            .entry(parse_number(y))
            .or_default()
            .insert(parse_number(x));
    }

    let pages = s2
        .lines()
        .map(|l| l.split(',').map(parse_number).collect::<Vec<_>>())
        .collect();

    (rules, pages)
}

#[aoc(day5, part1)]
fn part1_impl(input: &(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)) -> u32 {
    let (rules, pages) = input;

    pages
        .iter()
        .filter(|page| {
            page.is_sorted_by(|a, b| rules.get(b).map_or_else(|| false, |set| set.contains(a)))
        })
        .map(|page| page[page.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
fn part2_impl(input: &(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)) -> u32 {
    let (rules, pages) = input;

    let mut pages = pages.clone();

    pages
        .iter_mut()
        .filter(|page| {
            !page.is_sorted_by(|a, b| rules.get(b).map_or_else(|| false, |set| set.contains(a)))
        })
        .map(|page| {
            page.sort_by(|a, b| {
                rules
                    .get(b)
                    .map_or_else(|| false, |set| set.contains(a))
                    .cmp(&true)
            });
            page[page.len() / 2]
        })
        .sum()
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

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 123);
    }
}
