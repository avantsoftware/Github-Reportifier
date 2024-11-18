use chrono::NaiveDateTime;

use crate::models::Issue;

pub fn get_days_to_complete(issue: &Issue) -> i64 {
    let start_date = NaiveDateTime::parse_from_str(&issue.created_at, "%Y-%m-%dT%H:%M:%SZ")
        .unwrap()
        .date();

    let end_date = if let Some(closed_at_str) = &issue.closed_at {
        if !closed_at_str.is_empty() {
            NaiveDateTime::parse_from_str(closed_at_str, "%Y-%m-%dT%H:%M:%SZ")
                .unwrap()
                .date()
        } else {
            start_date
        }
    } else {
        start_date
    };

    let duration = (end_date - start_date).num_days() + 1;
    duration
}
