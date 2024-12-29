use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Repo {
    name: String,
}

pub async fn fetch_repos(username: &str, token: &str) {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/users/{}/repos", username);
    let response = client.get(&url).bearer_auth(token).send().await.unwrap();

    let repos: Vec<Repo> = repsonse.json().await.unwrap();
    for repo in repos {
        println!("Repo: {}", repo.name);
    }
}