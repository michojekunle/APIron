use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found in .env file");
    println!("GitHub Token: {}", github_token);
}