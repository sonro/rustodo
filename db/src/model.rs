use chrono::prelude::*;

#[derive(Queryable)]
pub struct TaskModel {
    id: i32,
    title: String,
    done: bool,
    updated_at: NaiveDateTime,
    due: Option<NaiveDateTime>,
    description: Option<String>,
}

impl From<TaskModel> for domain::Task {
    fn from(model: TaskModel) -> Self {
        Self {
            id: model.id,
            title: model.title,
            done: model.done,
            updated_at: DateTime::from_utc(model.updated_at, Utc),
            due: model.due.map(|d| DateTime::from_utc(d, Utc)),
            description: model.description,
        }
    }
}
