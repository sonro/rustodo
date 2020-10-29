use app::task;
use chrono::{Duration, Utc};
use clap::Clap;
use opts::Opts;

mod opts;
mod view;

fn main() {
    app::setup();
    let opts = Opts::parse();
    let task_repo = task::get_repo();
    match opts {
        Opts::List => list_tasks(task_repo),
        Opts::New => new_task(task_repo),
    }
}

fn list_tasks(task_repo: impl task::TaskRepository) {
    let tasks = task_repo.find_all();
    for task in tasks {
        dbg!(task);
    }
}

fn new_task(task_repo: impl task::TaskRepository) {
    use view::get_cli_input;

    let mut new_task = task::TaskForm::new();
    new_task.title(get_cli_input("Title: "));
    new_task.description(get_cli_input("Description:"));

    let due = get_cli_input("Hours until due: ");
    let due: i64 = due.parse().expect("parse due as integer");
    new_task.due(Utc::now() + Duration::hours(due));

    let task = task_repo.add(new_task);

    dbg!(&task);
}
