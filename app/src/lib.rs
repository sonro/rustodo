use inmem::repository::InMemTaskRepository;

mod inmem;

macro_rules! repo {
    ($mod_name:ident, $type:ty, $return:expr) => {
        pub mod $mod_name {
            pub use domain::task::*;
            pub type Repository = $type;
            pub fn create_repo() -> $type {
                $return
            }
        }
    };
}

repo!(task, db::DbTaskRepository, db::DbTaskRepository);

pub mod prelude {
    pub use crate::task::TaskRepository;
}

pub fn setup() -> Store {
    db::setup();
    Store::new()
}

pub struct Store {
    task_repo: task::Repository,
}

impl Store {
    fn new() -> Self {
        Self {
            task_repo: task::create_repo(),
        }
    }

    pub fn task_repo(&mut self) -> &mut task::Repository {
        &mut self.task_repo
    }
}

pub struct TestStore {
    task_repo: InMemTaskRepository,
}

impl TestStore {
    pub fn new() -> Self {
        Self {
            task_repo: InMemTaskRepository::new(),
        }
    }

    pub fn task_repo(&mut self) -> &mut InMemTaskRepository {
        &mut self.task_repo
    }
}
