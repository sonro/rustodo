use chrono::{DateTime, Utc};

pub trait Repository {
    type Item;
    type Updator;

    fn find_all(&self) -> Vec<Self::Item>;
    fn find_one(&self, id: i32) -> Self::Item;
    fn add(&self, updator: Self::Updator) -> Self::Item;
    fn remove(&self, id: i32) -> usize;
    fn update(&self, id: i32, updator: Self::Updator) -> Self::Item;
}

pub trait TaskRepository: Repository {}

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub updated_at: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

pub struct TaskForm {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub due: Option<Option<DateTime<Utc>>>,
    pub description: Option<Option<String>>,
}

impl TaskForm {
    pub fn new() -> Self {
        Self {
            title: None,
            done: None,
            due: None,
            description: None,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    pub fn done(&mut self) -> &mut Self {
        self.done = Some(true);
        self
    }

    pub fn remove_done(&mut self) -> &mut Self {
        self.done = Some(false);
        self
    }

    pub fn due(&mut self, due: DateTime<Utc>) -> &mut Self {
        self.due = Some(Some(due));
        self
    }

    pub fn remove_due(&mut self) -> &mut Self {
        self.due = Some(None);
        self
    }

    pub fn description(&mut self, description: String) -> &mut Self {
        self.description = Some(Some(description));
        self
    }

    pub fn remove_description(&mut self) -> &mut Self {
        self.description = Some(None);
        self
    }
}
