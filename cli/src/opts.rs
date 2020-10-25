use clap::Clap;

/// Todo app
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Christohper Morton <sonro@gmx.com>")]
pub enum Opts {
    /// List all tasks
    List,
    /// Create new task
    New,
}
