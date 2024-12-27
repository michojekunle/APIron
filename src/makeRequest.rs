use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found");

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", github_token))?,
    );

    let response = client
        .get("https://api.github.com/user")
        .headers(headers)
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("Response: {:?}", response);
    Ok(())
}
