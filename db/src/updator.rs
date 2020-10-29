use super::schema::tasks;
use chrono::NaiveDateTime;
use domain::task::TaskForm;

#[derive(Insertable, AsChangeset)]
#[table_name = "tasks"]
pub struct DbTaskUpdator {
    pub title: Option<String>,
    pub done: Option<bool>,
    pub due: Option<Option<NaiveDateTime>>,
    pub description: Option<Option<String>>,
}

impl From<TaskForm> for DbTaskUpdator {
    fn from(task: TaskForm) -> Self {
        Self {
            title: task.title,
            done: task.done,
            due: task.due.map(|d| d.map(|d| d.naive_utc())),
            description: task.description,
        }
    }
}
