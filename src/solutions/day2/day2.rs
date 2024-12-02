use std::fs::File;
use std::io::{self, BufRead};

fn get_input() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut return_vec: Vec<Vec<i32>> = Vec::new();

    let current_dir = std::env::current_dir()?;
    let file_path = current_dir
        .join("src")
        .join("solutions")
        .join("day2")
        .join("input.txt");

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s: &str| s.parse::<i32>().unwrap())
            .collect();

        return_vec.push(report);
    }
    Ok(return_vec)
}

fn is_safe_report(report: &[i32]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();

        if diff != 1 && diff != 2 && diff != 3 {
            return false;
        }

        if report[i] < report[i + 1] {
            decreasing = false;
        } else if report[i] > report[i + 1] {
            increasing = false;
        }
    }

    increasing || decreasing
}

pub fn part1() -> Result<i32, Box<dyn std::error::Error>> {
    let input = get_input()?;

    let mut total_safe_reports = 0;

    for report in input {
        if is_safe_report(&report) {
            total_safe_reports += 1;
        }
    }

    Ok(total_safe_reports)
}

pub fn part2() -> Result<i32, Box<dyn std::error::Error>> {
    let input = get_input()?;

    let mut total_safe_reports = 0;

    for report in input {
        if is_safe_report(&report) {
            total_safe_reports += 1;
            continue;
        }

        for i in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(i);

            if is_safe_report(&modified_report) {
                total_safe_reports += 1;
                break;
            }
        }
    }

    Ok(total_safe_reports)
}


