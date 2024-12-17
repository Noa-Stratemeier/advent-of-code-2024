/* LINK TO THE PROBLEM STATEMENT: https://adventofcode.com/2024/day/2 */

fn main() {
    // Solution for part 1
    part_one();

    // Solution for part 2
    part_two();
}

fn part_one() {
    let input = include_str!("../../src/day_2/day_2_input.txt");

    let mut safe_reports = 0;

    for line in input.lines() {
        let report: Vec<u32> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        if is_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
}

fn part_two() {
    let input = include_str!("../../src/day_2/day_2_input.txt");

    let mut safe_reports = 0;

    for line in input.lines() {
        let report: Vec<u32> = line.split(' ').map(|n| n.parse().unwrap()).collect();

        // Check if the report is safe without modifications
        if is_safe(&report) {
            safe_reports += 1;
            continue;
        }

        // Try removing each number and checking if it makes the report safe
        for i in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(i);

            if is_safe(&modified_report) {
                safe_reports += 1;
                break;
            }
        }
    }

    println!("Safe reports with tolerance: {safe_reports}");
}


/* Helper Functions */
fn is_safe(report: &[u32]) -> bool {
    let trend = report[1].cmp(&report[0]);

    for window in report.windows(2) {
        let current = window[0];
        let next = window[1];

        // Check if the difference of the current window is outside the allowed range
        if current.abs_diff(next) > 3 || current.abs_diff(next) < 1 {
            return false;
        }

        // Check if the current window's trend matches the report trend
        if next.cmp(&current) != trend {
            return false;
        }
    }

    true
}
