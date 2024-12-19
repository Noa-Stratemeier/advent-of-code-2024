/* LINK TO THE PROBLEM STATEMENT: https://adventofcode.com/2024/day/5 */

use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    // Solution for part 1 (~1.6ms)
    part_one();

    // Solution for part 2 (~3.2ms)
    part_two();
}

fn part_one() {
    let input = include_str!("../../src/day_5/day_5_input.txt");
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules into a set of pairs
    let rules: HashSet<(u32, u32)> = page_ordering_rules
        .lines()
        .map(|rule| rule.split_once('|').unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect();

    let compare = |x: &u32, y: &u32| !rules.contains(&(*y, *x));

    let mut sum_of_middle_pages = 0;

    for update in updates.lines() {
        let update: Vec<u32> = update.split(',').map(|x| x.parse().unwrap()).collect();

        if update.is_sorted_by(compare) {
            sum_of_middle_pages += update[update.len() / 2];
        }
    }

    println!("Sum of middle pages: {sum_of_middle_pages}");
}

pub fn part_two() {
    let input: &str = include_str!("../../src/day_5/day_5_input.txt");
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let orderings: HashSet<(u32, u32)> = orderings
        .lines()
        .map(|line| {
            let mut split = line.split('|');
            (split.next().unwrap().trim().parse().unwrap(), split.next().unwrap().trim().parse().unwrap())
        })
        .collect();

    let compare = |x: &u32, y: &u32| {
        if orderings.contains(&(*x, *y)) {
            Ordering::Less
        } else if orderings.contains(&(*y, *x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let sum_of_middle_pages: u32 = updates
        .lines()
        .filter_map(|update| {
            let mut pages: Vec<u32> = update.split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();

            if pages.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
                None
            } else {
                pages.sort_by(compare);
                Some(pages[pages.len() / 2])
            }
        })
        .sum();

    println!("Sum of middle pages for corrected updates: {sum_of_middle_pages}");
}
