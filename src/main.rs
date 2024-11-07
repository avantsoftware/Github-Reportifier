use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use chrono::{NaiveDate, NaiveDateTime};
use clap::Parser;
use std::error::Error;
use tabled::{Table, Tabled, Style};

#[derive(Debug, Deserialize, Serialize)]
struct SearchResult {
    items: Vec<Issue>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Issue {
    number: u64,
    title: String,
    body: Option<String>,
    user: Option<User>,
    created_at: String,
    closed_at: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    login: String,
}

#[derive(Tabled)]
struct PullRequestRow {
    #[tabled(rename = "#")]
    number: u64,
    #[tabled(rename = "Task")]
    title: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Dev")]
    developer: String,
    #[tabled(rename = "Repo")]
    repository: String,
    #[tabled(rename = "Start to End")]
    start_to_end: String,
    #[tabled(rename = "Work Days to Complete")]
    workdays: String,
}

#[derive(Parser)]
#[command(name = "GitHub PR Reporter")]
#[command(about = "Generates a report of GitHub pull requests for a given month.")]
struct Cli {
    #[arg(short, long, default_value_t = 2024)]
    year: i32,

    #[arg(short, long)]
    month: u32,

    #[arg(short, long, default_value = "table")]
    output: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args = Cli::parse();

    let github_token = env::var("GITHUB_TOKEN")
        .expect("GITHUB_TOKEN must be set in .env file or environment variables");
    let repo_owner = env::var("REPO_OWNER")
        .expect("REPO_OWNER must be set in .env file or environment variables");
    let repo_name = env::var("REPO_NAME")
        .expect("REPO_NAME must be set in .env file or environment variables");

    let client = Client::builder().user_agent("rust-lang").build()?;

    let prs = fetch_pull_requests(&client, &github_token, &repo_owner, &repo_name, &args)?;

    output_results(&prs, &repo_name, &args)?;

    Ok(())
}

fn fetch_pull_requests(
    client: &Client,
    github_token: &str,
    repo_owner: &str,
    repo_name: &str,
    args: &Cli,
) -> Result<Vec<Issue>, Box<dyn Error>> {
    let start_date = NaiveDate::from_ymd_opt(args.year, args.month, 1)
        .ok_or("Invalid start date")?;

    let end_date = if args.month == 12 {
        NaiveDate::from_ymd_opt(args.year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(args.year, args.month + 1, 1)
    }
    .ok_or("Invalid end date")?;

    let query = format!(
        "repo:{owner}/{repo} is:pr created:{start}..{end}",
        owner = repo_owner,
        repo = repo_name,
        start = start_date.format("%Y-%m-%d"),
        end = end_date.format("%Y-%m-%d")
    );

    let mut page = 1;
    let mut prs = Vec::new();

    loop {
        let url = format!(
            "https://api.github.com/search/issues?q={query}&per_page=100&page={page}",
            query = urlencoding::encode(&query),
            page = page
        );

        println!("Fetching page {}: {}", page, url);

        let response = client.get(&url).bearer_auth(&github_token).send()?;

        let status = response.status();
        let response_text = response.text()?;

        if !status.is_success() {
            eprintln!(
                "GitHub API request failed with status {}: {}",
                status, response_text
            );
            return Err(format!("GitHub API request failed with status {}", status).into());
        }

        let search_result = match serde_json::from_str::<SearchResult>(&response_text) {
            Ok(result) => result,
            Err(err) => {
                eprintln!(
                    "Failed to parse JSON response: {}\nResponse body: {}",
                    err, response_text
                );
                return Err(Box::new(err));
            }
        };

        if search_result.items.is_empty() {
            break;
        }

        prs.extend(search_result.items);
        page += 1;

        if page > 10 {
            break;
        }
    }

    Ok(prs)
}

fn output_results(
    prs: &Vec<Issue>,
    repo_name: &str,
    args: &Cli,
) -> Result<(), Box<dyn Error>> {
    if args.output.to_lowercase() == "json" {
        let json_output = serde_json::to_string_pretty(&prs)?;
        println!("{}", json_output);
    } else {
        let mut rows = Vec::new();

        for issue in prs.iter() {
            let description = issue
                .body
                .clone()
                .unwrap_or_else(|| "No description".to_string());
            let dev = issue.user.as_ref().map_or("Unknown".to_string(), |u| u.login.clone());
            let repo = repo_name.to_string();

            let start_date =
                NaiveDateTime::parse_from_str(&issue.created_at, "%Y-%m-%dT%H:%M:%SZ")?.date();

            let end_date = if let Some(closed_at_str) = &issue.closed_at {
                if !closed_at_str.is_empty() {
                    NaiveDateTime::parse_from_str(closed_at_str, "%Y-%m-%dT%H:%M:%SZ")?.date()
                } else {
                    start_date
                }
            } else {
                start_date
            };

            let duration = (end_date - start_date).num_days() + 1;

            let row = PullRequestRow {
                number: issue.number,
                title: issue.title.clone(),
                description: description.clone(),
                developer: dev.clone(),
                repository: repo.clone(),
                start_to_end: format!("{} - {}", start_date, end_date),
                workdays: duration.to_string(),
            };

            rows.push(row);
        }

        let table = Table::new(rows).with(Style::modern());

        println!("{}", table);
    }

    Ok(())
}

