mod cli;
mod github_api;
mod models;
mod output;

use dotenv::dotenv;
use reqwest::blocking::Client;
use std::error::Error;
use clap::Parser;

use crate::cli::Cli;
use crate::github_api::fetch_pull_requests;
use crate::output::output_results;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Cli::parse();

    let github_token = std::env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN must be set in .env file or environment variables");
    let repo_owner = std::env::var("REPO_OWNER")
        .expect("REPO_OWNER must be set in .env file or environment variables");
    let repo_name = std::env::var("REPO_NAME")
        .expect("REPO_NAME must be set in .env file or environment variables");

    let client = Client::builder().user_agent("rust-lang").build()?;

    let prs = fetch_pull_requests(&client, &github_token, &repo_owner, &repo_name, &args)?;
    output_results(&prs, &repo_name, &args)?;

    Ok(())
}

