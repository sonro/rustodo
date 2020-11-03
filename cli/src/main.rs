use clap::Clap;
use opts::Opts;

mod opts;

fn main() {
    let mut store = app::setup();
    let opts = Opts::parse();
    match opts {
        Opts::List => cli::list_tasks(&mut store),
        Opts::New => cli::new_task(&mut store),
    }
}
