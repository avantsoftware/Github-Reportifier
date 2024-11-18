use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub items: Vec<Issue>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub user: Option<User>,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub login: String,
}

pub struct SummaryFields {
    pub fix: usize,
    pub feat: usize,
    pub refactor: usize,
    pub test: usize,
    pub build: usize,
    pub chore: usize,
}

#[derive(Tabled)]
pub struct PullRequestRow {
    #[tabled(rename = "#")]
    pub number: u64,
    #[tabled(rename = "Task")]
    pub title: String,
    #[tabled(rename = "Description")]
    pub description: String,
    #[tabled(rename = "Dev")]
    pub developer: String,
    #[tabled(rename = "Repo")]
    pub repository: String,
    #[tabled(rename = "Start to End")]
    pub start_to_end: String,
    #[tabled(rename = "Work Days to Complete")]
    pub workdays: String,
    #[tabled(rename = "PR URL")]
    pub url: String,
}
