/* LINK TO THE PROBLEM STATEMENT: https://adventofcode.com/2024/day/3 */

use regex::Regex;

fn main() {
    // Solution for part 1 (~4.5ms)
    part_one();

    // Solution for part 2 (~6.5ms)
    part_two();
}

fn part_one() {
    // Creates a string from the input
    let input = include_str!("../../src/day_3/day_3_input.txt");

    // Define a regular expression to match 'mul(int,int)' and capture the integers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum_of_products = 0;

    // Iterate through all matches
    for caps in re.captures_iter(input) {
        // Extract the matched integers
        let int1: i32 = caps[1].parse().unwrap();
        let int2: i32 = caps[2].parse().unwrap();

        sum_of_products += int1 * int2
    }

    println!("Sum of products: {sum_of_products}");
}

fn part_two() {
    let input = include_str!("../../src/day_3/day_3_input.txt");

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut mul_enabled: bool = true;
    let mut sum_of_products = 0;

    for caps in re.captures_iter(input) {
        if let Some(int1_match) = caps.get(1) {
            if mul_enabled {
                let int1: i32 = int1_match.as_str().parse().unwrap();
                let int2: i32 = caps[2].parse().unwrap();

                sum_of_products += int1 * int2;
            }
        } else if caps.get(0).unwrap().as_str() == "do()" {
            mul_enabled = true;
        } else if caps.get(0).unwrap().as_str() == "don't()" {
            mul_enabled = false;
        }
    }

    println!("Sum of products with conditionals: {sum_of_products}");
}
