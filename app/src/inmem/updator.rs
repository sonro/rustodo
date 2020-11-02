use chrono::prelude::*;
use domain::task::{Task, TaskForm};

pub struct InMemTaskUpdator {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub due: Option<Option<DateTime<Utc>>>,
    pub description: Option<Option<String>>,
}

impl super::Insertable<Task> for InMemTaskUpdator {
    fn to_item(self, id: i32) -> Task {
        let title = self.title.expect("Task needs a title");
        let done = self.done.unwrap_or(false);
        let updated_at = Utc::now();
        let due = match self.due {
            None | Some(None) => None,
            Some(Some(d)) => Some(d),
        };
        let description = match self.description {
            None | Some(None) => None,
            Some(Some(d)) => Some(d),
        };
        Task {
            id,
            title,
            done,
            updated_at,
            due,
            description,
        }
    }

    fn update_item(self, item: &mut Task) {
        if let Some(title) = self.title {
            item.title = title;
        }
        if let Some(done) = self.done {
            item.done = done;
        }
        if let Some(due) = self.due {
            item.due = due;
        }
        if let Some(description) = self.description {
            item.description = description;
        }
        item.updated_at = Utc::now();
    }
}

impl From<TaskForm> for InMemTaskUpdator {
    fn from(task: TaskForm) -> Self {
        Self {
            title: task.title,
            done: task.done,
            due: task.due,
            description: task.description,
        }
    }
}
