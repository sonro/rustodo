use clap::Clap;
use opts::Opts;

mod opts;
mod view;

fn main() {
    let opts = Opts::parse();

    match opts {
        Opts::List => list_tasks(),
        Opts::New => new_task(),
    }
}

pub fn list_tasks() {
    // let tasks = Task::find_all();
    // for task in tasks {
    //     dbg!(task);
    // }
}

pub fn new_task() {
    // let title = get_cli_input("Title: ");
    // let description = get_cli_input("Description: ");

    // let due = get_cli_input("Hours until due: ");
    // let due: i64 = due.parse().expect("parse due as integer");
    // let due = Utc::now() + Duration::hours(due);

    // let mut new_task = TaskDto::new(title);
    // new_task.description(description);
    // new_task.due(due.naive_utc());

    // let task = Task::add(&new_task);

    // dbg!(&task);
}
