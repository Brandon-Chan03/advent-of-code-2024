use dotenv::dotenv;
use std::env;

use reqwest;

use std::fs;
use std::io::Write;
use std::path::Path;

pub async fn fetch_input(year: i32, day: i32) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();

    let session_token = env::var("SESSION_COOKIE")?;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    
    let response = reqwest::Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()
        .await?
        .text()
        .await?;

    let folder_path = format!("src/solutions/day{}", day);
    fs::create_dir_all(&folder_path)?;
    let file_path = Path::new(&folder_path).join("input.txt");
    let mut file = fs::File::create(file_path)?;
    file.write_all(response.as_bytes())?;

    Ok(())
}