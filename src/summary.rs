use std::collections::HashMap;

use crate::{models::Issue, utils::get_days_to_complete};
use regex::Regex;

#[derive(Debug)]
struct PrDetails {
    count: usize,
    days_to_complete: i64,
}

impl PrDetails {
    pub fn update(&mut self, days_to_complete: i64) {
        self.days_to_complete += days_to_complete;
        self.count += 1;
    }
}

#[derive(Debug)]
struct SummaryFields {
    fix: PrDetails,
    feat: PrDetails,
    refactor: PrDetails,
    test: PrDetails,
    build: PrDetails,
    chore: PrDetails,
    unlabeled: PrDetails,
}

impl SummaryFields {
    pub fn new() -> Self {
        Self {
            fix: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            feat: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            refactor: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            test: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            build: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            chore: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
            unlabeled: PrDetails {
                count: 0,
                days_to_complete: 0,
            },
        }
    }
    pub fn update(&mut self, key: &str, days_to_complete: i64) {
        match key {
            "fix" => self.fix.update(days_to_complete),
            "feat" => self.feat.update(days_to_complete),
            "refactor" => self.refactor.update(days_to_complete),
            "test" => self.test.update(days_to_complete),
            "build" => self.build.update(days_to_complete),
            "chore" => self.chore.update(days_to_complete),
            _ => self.unlabeled.update(days_to_complete),
        }
    }
}

pub fn categorize_pr(title: &String) -> Option<String> {
    let patterns = [
        (Regex::new(r"(?i)\s*fix:").unwrap(), "fix"),
        (Regex::new(r"(?i)\s*feat:").unwrap(), "feat"),
        (Regex::new(r"(?i)\s*refactor:").unwrap(), "refactor"),
        (Regex::new(r"(?i)\s*test:").unwrap(), "test"),
        (Regex::new(r"(?i)\s*build:").unwrap(), "build"),
        (Regex::new(r"(?i)\s*chore:").unwrap(), "chore"),
    ];

    println!("Title: {}", title);

    for (regex, category) in &patterns {
        if regex.is_match(title) {
            return Some(category.to_string());
        }
    }

    None
}

pub fn get_prs_summary(prs: &Vec<Issue>) {
    let mut user_prs: HashMap<String, SummaryFields> = HashMap::new();

    for pr in prs {
        let pr_type = categorize_pr(&pr.title).unwrap_or("unlabeled".to_string());

        let user_login = pr
            .user
            .as_ref()
            .map(|user| user.login.clone())
            .unwrap_or("Unknown Contributor".to_string());

        let user_summary = user_prs
            .entry(user_login)
            .or_insert_with(SummaryFields::new);

        let duration = get_days_to_complete(pr);
        user_summary.update(pr_type.as_str(), duration);
    }

}
