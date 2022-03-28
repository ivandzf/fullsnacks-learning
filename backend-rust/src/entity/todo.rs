use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    #[serde(deserialize_with = "deserialize_nullable_datetime", serialize_with = "serialize_nullable_datetime")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_datetime", serialize_with = "serialize_datetime")]
    pub created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            completed: false,
            completed_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn acknowledge(&mut self) {
        self.completed = true;
        self.completed_at = Some(Utc::now());
    }
}

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, DATETIME_FORMAT)
        .map_err(serde::de::Error::custom)
}

fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format(DATETIME_FORMAT).to_string())
}

fn deserialize_nullable_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Utc.datetime_from_str(&s, DATETIME_FORMAT)
            .map_err(serde::de::Error::custom)
            .map(Some)
    }
}

fn serialize_nullable_datetime<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(date) => serializer.serialize_str(&date.format(DATETIME_FORMAT).to_string()),
        None => serializer.serialize_str(""),
    }
}

#[derive(Deserialize, Serialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: vec![] }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
