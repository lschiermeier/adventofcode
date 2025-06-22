use reqwest::Client;
use std::{fs, env, time::Duration};
use dotenv::dotenv;

const BASE_URL: &str = "https://adventofcode.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let session = env::var("AOC_SESSION").expect("Missing AOC_SESSION in .env");

    let year = env::args().nth(1).expect("Usage: cargo run <YEAR>");
    let year: u32 = year.parse().expect("Year must be a number");

    let client = Client::builder()
        .cookie_store(true)
        .user_agent("github.com/your-username/aoc-downloader")
        .timeout(Duration::from_secs(10))
        .build()?;

    let cookie = format!("session={}", session);

    for day in 1..=25 {
        let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);
        println!("Downloading Day {}...", day);

        let response = client
            .get(&url)
            .header("Cookie", &cookie)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let dir = format!("../{}/inputs/", year);
            fs::create_dir_all(&dir)?;
            let path = format!("{}day{:02}.txt", dir, day);
            fs::write(&path, body)?;
            println!("Saved to {}", path);
        } else {
            eprintln!("Failed to download Day {}: {}", day, response.status());
        }
    }

    println!("✅ All available inputs downloaded.");
    Ok(())
}
