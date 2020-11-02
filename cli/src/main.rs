use app::prelude::*;
use app::task;
use chrono::{Duration, Utc};
use clap::Clap;
use opts::Opts;

mod opts;
mod view;

fn main() {
    let mut store = app::setup();
    let opts = Opts::parse();
    match opts {
        Opts::List => list_tasks(&mut store),
        Opts::New => new_task(&mut store),
    }
}

fn list_tasks(store: &mut app::Store) {
    let task_repo = store.task_repo();
    let tasks = task_repo.find_all();
    for task in tasks {
        dbg!(task);
    }
}

fn new_task(store: &mut app::Store) {
    let task_repo = store.task_repo();
    use view::get_cli_input;

    let mut new_task = task::TaskForm::new();
    new_task.title(get_cli_input("Title: "));
    new_task.description(get_cli_input("Description: "));

    let due = get_cli_input("Hours until due: ");
    let due: i64 = due.parse().expect("parse due as integer");
    new_task.due(Utc::now() + Duration::hours(due));

    let task = task_repo.add(new_task);

    dbg!(&task);
}
