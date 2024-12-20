use crate::cli::Cli;
use crate::models::{Issue, PullRequestRow};
use crate::utils::get_days_to_complete;
use chrono::NaiveDateTime;
use serde::Serialize;
use std::error::Error;
use tabled::{object::Columns, Modify, Style, Table, Width};

fn print_to_json<T>(arg: &T) -> Result<(), Box<dyn Error>>
where
    T: ?Sized + Serialize,
{
    let json = serde_json::to_string_pretty(arg)?;
    println!("{}", json);
    Ok(())
}

fn print_to_terminal(prs: &[Issue], repo_name: &str) -> Result<(), Box<dyn Error>> {
    let mut rows = Vec::new();

    for issue in prs.iter() {
        let description = issue
            .body
            .clone()
            .unwrap_or_else(|| "No description".to_string());
        let dev = issue
            .user
            .as_ref()
            .map_or("Unknown".to_string(), |u| u.login.clone());

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

        let url = issue.html_url.clone();

        let duration = get_days_to_complete(&issue);

        let row = PullRequestRow {
            number: issue.number,
            title: issue.title.clone(),
            description: description.clone(),
            developer: dev.clone(),
            repository: repo_name.to_string(),
            start_to_end: format!("{} - {}", start_date, end_date),
            workdays: duration.to_string(),
            url: url.to_string(),
        };

        rows.push(row);
    }

    let table = Table::new(rows)
        .with(Style::modern())
        .with(Modify::new(Columns::single(1)).with(Width::wrap(30).keep_words()))
        .with(Modify::new(Columns::single(2)).with(Width::wrap(40).keep_words()))
        .with(Modify::new(Columns::single(3)).with(Width::wrap(40).keep_words()))
        .with(Modify::new(Columns::single(4)).with(Width::truncate(25).suffix("...")))
        .with(Modify::new(Columns::single(5)).with(Width::truncate(25).suffix("...")))
        .with(Modify::new(Columns::single(6)).with(Width::truncate(5).suffix("...")));

    println!("{}", &table);
    Ok(())
}

pub fn output_results(prs: &[Issue], repo_name: &str, args: &Cli) -> Result<(), Box<dyn Error>> {
    let output_type = args.output.to_lowercase();

    match output_type.as_str() {
        "json" => print_to_json(prs),
        _ => print_to_terminal(prs, repo_name),
    }
}
