use sled_extensions::Db;

use crate::entity::todo::{TodoList, Todo};

pub fn get_all_todos(db: &Db) -> TodoList {
    let mut todos = TodoList::new();
    todos
}

pub fn create(db: &Db, title: String) -> Result<(), std::fmt::Error> {
    Err(std::fmt::Error::default())
}
