use super::connection::get_conn;
use super::model::TaskModel;
use super::schema::tasks;
use super::updator::DbTaskUpdator;
use diesel::prelude::*;
use domain::task::*;

pub struct DbTaskRepository;

fn models_into_tasks(models: Vec<TaskModel>) -> Vec<Task> {
    models.into_iter().map(|t| t.into()).collect()
}

impl TaskRepository for DbTaskRepository {
    fn find_all(&self) -> Vec<Task> {
        let models = tasks::table.load(&get_conn()).expect("find all tasks");
        models_into_tasks(models)
    }

    fn find_one(&self, id: i32) -> Task {
        let model: TaskModel = tasks::table.find(id).first(&get_conn()).expect("find task");
        model.into()
    }

    fn add(&mut self, form: TaskForm) -> Task {
        let form: DbTaskUpdator = form.into();
        let model: TaskModel = diesel::insert_into(tasks::table)
            .values(form)
            .get_result(&get_conn())
            .expect("save new task");
        model.into()
    }

    fn remove(&mut self, id: i32) -> usize {
        diesel::delete(tasks::table.filter(tasks::id.eq(id)))
            .execute(&get_conn())
            .expect("delete task")
    }

    fn update(&mut self, id: i32, form: TaskForm) -> Task {
        let updator: DbTaskUpdator = form.into();
        let model: TaskModel = diesel::update(tasks::table)
            .filter(tasks::id.eq(id))
            .set(updator)
            .get_result(&get_conn())
            .expect("update task");
        model.into()
    }
}
