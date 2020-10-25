use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub updated_at: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

pub struct TaskUpdator {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub due: Option<Option<DateTime<Utc>>>,
    pub description: Option<Option<String>>,
}
