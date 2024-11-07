use crate::cli::Cli;
use crate::models::{Issue, SearchResult};
use chrono::NaiveDate;
use reqwest::blocking::Client;
use std::error::Error;

pub fn fetch_pull_requests(
    client: &Client,
    github_token: &str,
    repo_owner: &str,
    repo_name: &str,
    args: &Cli,
) -> Result<Vec<Issue>, Box<dyn Error>> {
    let (start_date, end_date) = calculate_date_range(args)?;
    let query = build_search_query(repo_owner, repo_name, start_date, end_date);

    let mut prs = Vec::new();
    let mut page = 1;

    loop {
        let search_result = fetch_search_results(client, github_token, &query, page)?;

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

fn calculate_date_range(args: &Cli) -> Result<(NaiveDate, NaiveDate), Box<dyn Error>> {
    let start_date = NaiveDate::from_ymd_opt(args.year, args.month, 1)
        .ok_or("Invalid start date")?;

    let end_date = if args.month == 12 {
        NaiveDate::from_ymd_opt(args.year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(args.year, args.month + 1, 1)
    }
    .ok_or("Invalid end date")?;

    Ok((start_date, end_date))
}

fn build_search_query(
    repo_owner: &str,
    repo_name: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> String {
    format!(
        "repo:{owner}/{repo} is:pr created:{start}..{end}",
        owner = repo_owner,
        repo = repo_name,
        start = start_date.format("%Y-%m-%d"),
        end = end_date.format("%Y-%m-%d")
    )
}

fn fetch_search_results(
    client: &Client,
    github_token: &str,
    query: &str,
    page: u32,
) -> Result<SearchResult, Box<dyn Error>> {
    let url = build_request_url(query, page);

    println!("Fetching page {}: {}", page, url);

    let response = client.get(&url).bearer_auth(github_token).send()?;
    let status = response.status();
    let response_text = response.text()?;

    if !status.is_success() {
        eprintln!(
            "GitHub API request failed with status {}: {}",
            status, response_text
        );
        return Err(format!("GitHub API request failed with status {}", status).into());
    }

    let search_result: SearchResult = serde_json::from_str(&response_text).map_err(|err| {
        eprintln!(
            "Failed to parse JSON response: {}\nResponse body: {}",
            err, response_text
        );
        err
    })?;

    Ok(search_result)
}

fn build_request_url(query: &str, page: u32) -> String {
    format!(
        "https://api.github.com/search/issues?q={query}&per_page=100&page={page}",
        query = urlencoding::encode(query),
        page = page
    )
}

