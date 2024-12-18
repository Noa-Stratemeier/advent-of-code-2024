/* LINK TO THE PROBLEM STATEMENT: https://adventofcode.com/2024/day/4 */

use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // Solution for part 1 (~3.5ms)
    part_one();

    // Solution for part 2 (~1.3ms)
    part_two();
}

fn part_one() {
    let word_search_matrix: Vec<Vec<char>> = read_in_word_search();

    let mut xmas_count = 0;

    let directions = [
        (-1, 0), // up
        (1, 0),  // down
        (0, -1), // left
        (0, 1),  // right
        (-1, -1), // up-Left
        (-1, 1),  // up-Right
        (1, -1),  // down-Left
        (1, 1),   // down-Right
    ];

    for row in 0..word_search_matrix.len() {
        for col in 0..word_search_matrix[0].len() {
            if word_search_matrix[row][col] == 'X' {
                for &(d_row, d_col) in &directions {
                    if check_direction(&word_search_matrix, row, col, d_row, d_col) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    println!("XMAS count: {xmas_count}");
}

fn part_two() {
    let word_search_matrix: Vec<Vec<char>> = read_in_word_search();

    let mut xmases_count = 0;

    let rows = word_search_matrix.len();
    let cols = word_search_matrix[0].len();

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if word_search_matrix[row][col] == 'A' {
                if matches_xmases_pattern(&word_search_matrix, row, col) {
                    xmases_count += 1;
                }
            }
        }
    }

    println!("X-MASes count: {xmases_count}");
}


/* Helper Functions */
fn read_in_word_search() -> Vec<Vec<char>> {
    let file = File::open("./src/day_4/day_4_input.txt").expect("failed to open file");
    let lines = io::BufReader::new(file).lines();

    let mut word_search_matrix: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        let row: Vec<char> = line.chars().collect();
        word_search_matrix.push(row);
    }

    word_search_matrix
}

fn check_direction(matrix: &Vec<Vec<char>>, start_row: usize, start_col: usize, d_row: isize, d_col: isize) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    for (k, ch) in "MAS".chars().enumerate() {
        let new_row = start_row as isize + (k+1) as isize * d_row;
        let new_col = start_col as isize + (k+1) as isize * d_col;

        // Check if new position is out of bounds
        if new_row < 0 || new_row >= rows as isize || new_col < 0 || new_col >= cols as isize {
            return false;
        }

        // Check if the character matches
        if matrix[new_row as usize][new_col as usize] != ch {
            return false;
        }
    }

    true
}

fn matches_xmases_pattern(matrix: &Vec<Vec<char>>, middle_row: usize, middle_col: usize) -> bool {
    // Define the four corners of the 3x3 kernel around the center (middle_row, middle_col)
    let top_left = matrix[middle_row - 1][middle_col - 1];
    let top_right = matrix[middle_row - 1][middle_col + 1];
    let bottom_left = matrix[middle_row + 1][middle_col - 1];
    let bottom_right = matrix[middle_row + 1][middle_col + 1];

    // Ensure all corners are either 'S' or 'M'
    let valid_corners = |c: char| c == 'S' || c == 'M';

    if !valid_corners(top_left) || !valid_corners(top_right) || !valid_corners(bottom_left) || !valid_corners(bottom_right) {
        return false;
    }

    // Ensure opposite corners are not the same
    if top_left == bottom_right || top_right == bottom_left {
        return false;
    }

    true
}
