use super::updator::InMemTaskUpdator;
use super::*;
use domain::task::*;

pub struct InMemTaskRepository(MemStore<Task>);

impl InMemTaskRepository {
    pub fn new() -> Self {
        Self(MemStore::<Task>::new())
    }
}

impl TaskRepository for InMemTaskRepository {
    fn find_all(&self) -> Vec<Task> {
        self.0.fetch_all()
    }

    fn find_one(&self, id: i32) -> Task {
        self.0.fetch_one(id).unwrap()
    }

    fn add(&mut self, form: TaskForm) -> Task {
        let form: InMemTaskUpdator = form.into();
        self.0.insert(form)
    }

    fn remove(&mut self, id: i32) -> usize {
        match self.0.delete(id) {
            Some(_) => 1,
            None => 0,
        }
    }

    fn update(&mut self, id: i32, form: TaskForm) -> Task {
        let form: InMemTaskUpdator = form.into();
        self.0.update(id, form)
    }
}
