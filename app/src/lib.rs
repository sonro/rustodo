mod inmem;

pub fn setup() {
    db::setup();
}

macro_rules! repo {
    ($mod_name:ident, $type:ty, $return:expr) => {
        pub mod $mod_name {
            pub use domain::$mod_name::*;
            pub fn get_repo() -> $type {
                $return
            }
        }
    };
}

repo!(task, db::DbTaskRepository, db::DbTaskRepository);
