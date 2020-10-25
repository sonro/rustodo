use chrono::{Duration, Utc};
use clap::Clap;
use opts::Opts;
use todo::*;

mod opts;
mod view;

fn main() {
    app::setup();
    let opts = Opts::parse();

    match opts {
        Opts::List => list_tasks(),
        Opts::New => new_task(),
    }
}

fn list_tasks() {
    let repo = app::task::get_repo();
    let tasks = repo.find_all();
    for task in tasks {
        dbg!(task);
    }
}

fn new_task() {
    use view::get_cli_input;

    let mut new_task = TaskForm::new();
    new_task.title(get_cli_input("Title: "));
    new_task.description(get_cli_input("Description:"));

    let due = get_cli_input("Hours until due: ");
    let due: i64 = due.parse().expect("parse due as integer");
    new_task.due(Utc::now() + Duration::hours(due));

    let repo = app::task::get_repo();
    let task = repo.add(new_task);

    dbg!(&task);
}
