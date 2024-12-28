use dotenv::dotenv;
use std::env;

fn load_api_key() -> String {
    dotenv().ok();
    env::var("GITHUB_API_KEY").expect("GITHUB_API_KEY not set")
}