use crate::cli::Cli;
use crate::models::{Issue, PullRequestRow};
use chrono::NaiveDateTime;
use std::error::Error;
use tabled::{object::Columns, Modify, Style, Table, Width};

pub fn output_results(
    prs: &[Issue],
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
            let dev = issue
                .user
                .as_ref()
                .map_or("Unknown".to_string(), |u| u.login.clone());

            let start_date = NaiveDateTime::parse_from_str(&issue.created_at, "%Y-%m-%dT%H:%M:%SZ")?
                .date();

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

            let duration = (end_date - start_date).num_days() + 1;

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
    }

    Ok(())
}

