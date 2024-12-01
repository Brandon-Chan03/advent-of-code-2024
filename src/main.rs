use std::io::{self, Write};

use utils::fetch;

mod utils;

#[tokio::main]
async fn run_fetch(year: i32, number: i32) {
    match fetch::fetch_input(year, number).await {
        Ok(_) => println!("Hello"),
        Err(e) => println!("ERROR: {}", e),
    }

}

fn main() {
    let mut input = String::new();
    let number: i32;
    let year: i32;

    loop {
        input.clear();
        println!("Menu:");
        println!("1. View solutions");
        println!("2. Fetch input");
        print!("Enter your choice (1 or 2): ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                loop {
                    input.clear();
                    print!("Enter the day you want (1-25): ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    match input.trim().parse() {
                        Ok(num) if num >= 1 && num <= 25 => {
                            number = num;
                            println!("Your number is {}", number);
                            break;
                        }
                        _ => println!("Invalid input. Please enter a number between 1 and 25."),
                    }
                }
                break;
            }
            "2" => {
                loop {
                    input.clear();
                    print!("Enter the year (2024): ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    match input.trim().parse() {
                        Ok(num) if num >= 2024 => {
                            year = num;
                            break;
                        }
                        _ => println!("Invalid input. Please enter a year 2024 or later."),
                    }
                }

                loop {
                    input.clear();
                    print!("Enter the day you want (1-25): ");
                    io::stdout().flush().expect("Failed to flush stdout");
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    match input.trim().parse() {
                        Ok(num) if num >= 1 && num <= 25 => {
                            number = num;
                            break;
                        }
                        _ => println!("Invalid input. Please enter a number between 1 and 25."),
                    }
                }
                run_fetch(year, number);
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}
