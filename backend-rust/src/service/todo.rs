use std::fmt::Error;

use backend_rust::{InMemory, InMemoryStore};

use crate::entity::todo::{TodoList, Todo};

pub fn get_all_todos(store: &InMemory) -> TodoList {
    let mut todos = TodoList::new();
    store.get_all().iter().for_each(|v| {
        let todo: Todo = serde_json::from_str(v).unwrap();
        todos.add(todo);
    });

    todos
}

pub fn create(store: &mut InMemory, title: String) -> Result<(), Error> {
    Ok(store.add("1".to_string(), title))
}
