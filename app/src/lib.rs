type TaskRepository = db::DbTaskRepository;

pub fn setup() {
    db::setup();
}

struct Repositories {
    task: TaskRepository,
}

impl Repositories {
    fn init() -> Self {
        Self {
            task: db::DbTaskRepository,
        }
    }
}

pub mod task {
    use super::*;

    pub fn get_repo() -> TaskRepository {
        Repositories::init().task
    }
}
