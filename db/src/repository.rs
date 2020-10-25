use super::connection::get_conn;
use super::model::TaskModel;
use super::schema::tasks;
use super::updator::DbTaskUpdator;
use diesel::prelude::*;
use domain::*;

pub struct DbTaskRepository;

fn models_into_tasks(models: Vec<TaskModel>) -> Vec<Task> {
    models.into_iter().map(|t| t.into()).collect()
}

impl Repository for DbTaskRepository {
    type Item = Task;
    type Updator = TaskForm;

    fn find_all(&self) -> Vec<Self::Item> {
        let models = tasks::table.load(&get_conn()).expect("find all tasks");
        models_into_tasks(models)
    }

    fn find_one(&self, id: i32) -> Self::Item {
        let model: TaskModel = tasks::table.find(id).first(&get_conn()).expect("find task");
        model.into()
    }

    fn add(&self, updator: Self::Updator) -> Self::Item {
        let updator: DbTaskUpdator = updator.into();
        let model: TaskModel = diesel::insert_into(tasks::table)
            .values(updator)
            .get_result(&get_conn())
            .expect("save new task");
        model.into()
    }

    fn remove(&self, id: i32) -> usize {
        diesel::delete(tasks::table.filter(tasks::id.eq(id)))
            .execute(&get_conn())
            .expect("delete task")
    }

    fn update(&self, id: i32, updator: Self::Updator) -> Self::Item {
        let updator: DbTaskUpdator = updator.into();
        let model: TaskModel = diesel::update(tasks::table)
            .filter(tasks::id.eq(id))
            .set(updator)
            .get_result(&get_conn())
            .expect("update task");
        model.into()
    }
}
