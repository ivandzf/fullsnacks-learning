use chrono::NaiveDateTime;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub completed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            completed: false,
            completed_at: NaiveDateTime::from_timestamp(0, 0),
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn acknowledge(&mut self) {
        self.completed = true;
        self.completed_at = chrono::Utc::now().naive_utc();
    }
}
