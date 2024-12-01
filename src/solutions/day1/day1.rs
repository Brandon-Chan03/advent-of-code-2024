use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn get_input() -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut return_vec: Vec<Vec<i32>> = Vec::new();

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
    return_vec.push(left_column);
    return_vec.push(right_column);

    Ok(return_vec)
}

pub fn part1() -> Result<i32, Box<dyn std::error::Error>> {
    
    let input = get_input();
    
    let input = input?;
    let mut left_column = input.get(0).unwrap().clone();
    let mut right_column = input.get(1).unwrap().clone();

    left_column.sort();
    right_column.sort();

    let mut total_difference = 0;

    for (left, right) in left_column.iter().zip(right_column.iter()) {
        total_difference += (left - right).abs();
    }

    Ok(total_difference)
}

pub fn part2() -> Result<i32, Box<dyn std::error::Error>> {
    
    let input = get_input();
    
    let input = input?;
    let left_column = input.get(0).unwrap().clone();
    let right_column = input.get(1).unwrap().clone();

    let mut count_map = HashMap::new();
    for num in right_column {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut total = 0;
    for num in left_column {
        if let Some(&count) = count_map.get(&num) {
            total += num * count;
        }
    }

    Ok(total)
}