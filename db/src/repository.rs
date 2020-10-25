use super::connection::get_conn;
use super::model::TaskModel;
use super::schema::tasks;
use super::updator::DbTaskUpdator;
use diesel::prelude::*;

pub trait DbRepository {
    type Item;
    type Updator;

    fn find_all() -> Vec<Self::Item>;
    fn find_one(id: i32) -> Self::Item;
    fn add(updator: Self::Updator) -> Self::Item;
    fn remove(id: i32) -> usize;
    fn update(id: i32, updator: Self::Updator) -> Self::Item;
}

pub struct DbTaskRepository;

impl DbRepository for DbTaskRepository {
    type Item = TaskModel;
    type Updator = DbTaskUpdator;

    fn find_all() -> Vec<Self::Item> {
        tasks::table.load(&get_conn()).expect("find all tasks")
    }

    fn find_one(id: i32) -> Self::Item {
        tasks::table.find(id).first(&get_conn()).expect("find task")
    }

    fn add(updator: Self::Updator) -> Self::Item {
        diesel::insert_into(tasks::table)
            .values(updator)
            .get_result(&get_conn())
            .expect("save new task")
    }

    fn remove(id: i32) -> usize {
        diesel::delete(tasks::table.filter(tasks::id.eq(id)))
            .execute(&get_conn())
            .expect("delete task")
    }

    fn update(id: i32, updator: Self::Updator) -> Self::Item {
        diesel::update(tasks::table)
            .filter(tasks::id.eq(id))
            .set(updator)
            .get_result(&get_conn())
            .expect("update task")
    }
}
