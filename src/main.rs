mod cli;
use dotenv::dotenv;
use std::env;
use utils::load_api_key;

fn main() {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        Some(("github", github_matches)) => {
            if let Some(("list-repos", args)) = github_matches.subcommand() {
                let username = args.get_one::<String>("username").unwrap();
                println!("Fetching repos for {}", username);
                github::fetch_repos(username, &load_api_key()).await;
            }
        }
        _ => println!("Unknown command"),
    }
}