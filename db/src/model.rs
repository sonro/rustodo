use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct TaskModel {
    id: i32,
    title: String,
    done: bool,
    updated_at: NaiveDateTime,
    due: Option<NaiveDateTime>,
    description: Option<String>,
}

impl From<todo::Task> for TaskModel {
    fn from(task: todo::Task) -> Self {
        Self {
            id: task.id,
            title: task.title,
            done: task.done,
            updated_at: task.updated_at.naive_utc(),
            due: task.due.map(|d| d.naive_utc()),
            description: task.description,
        }
    }
}

impl From<&todo::Task> for TaskModel {
    fn from(task: &todo::Task) -> Self {
        Self {
            id: task.id,
            title: task.title.clone(),
            done: task.done,
            updated_at: task.updated_at.naive_utc(),
            due: task.due.map(|d| d.naive_utc()),
            description: task.description.clone(),
        }
    }
}
