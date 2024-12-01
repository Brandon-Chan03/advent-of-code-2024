use std::fs::File;
use std::io::{self, BufRead};

pub fn read_input() -> Result<i32, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir
    .join("src")
    .join("solutions")
    .join("day1")
    .join("input.txt");

    let mut left_column: Vec<i32> = Vec::new();
    let mut right_column: Vec<i32> = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            if let (Ok(left), Ok(right)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                left_column.push(left);
                right_column.push(right);
            }
        }
    }

    left_column.sort();
    right_column.sort();

    let mut total_difference = 0;

    for (left, right) in left_column.iter().zip(right_column.iter()) {
        total_difference += (left - right).abs();
    }

    Ok(total_difference)
}