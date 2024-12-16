/* LINK TO THE PROBLEM STATEMENT: https://adventofcode.com/2024/day/1 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn main() {
    // Solution for part 1
    part_one();

    // Solution for part 2
    part_two();
}


fn part_one() {
    let (mut left_list, mut right_list) = read_in_lists();

    // Sort both lists in ascending order
    left_list.sort();
    right_list.sort();

    // Calculate the sum of absolute differences between corresponding elements of the sorted lists
    let total_distance: i32 = left_list.iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    println!("Total distance between the lists: {total_distance}");
}


fn part_two() {
    let (left_list, right_list) = read_in_lists();

    // Count occurrences of each number in the right list
    let mut right_counts = HashMap::new();
    for num in right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score: for each number in the left list,
    // multiply it by the number of times it appears in the right list, then sum the results
    let similarity: i32 = left_list.iter()
        .map(|num| num * right_counts.get(num).unwrap_or(&0))
        .sum();

    println!("Similarity score: {similarity}");
}



/* Helper Functions */
fn read_in_lists() -> (Vec<i32>, Vec<i32>) {
    // Read in the input .txt file
    let file = File::open("src/day_1/day_1_input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines().flatten() {
        // Parse the line into two numbers and add them to their respective lists,
        // an example line from the .txt file looks like this: "84283   63343"
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    }

    (left_list, right_list)
}
