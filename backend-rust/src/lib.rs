pub struct AppState {
    pub store: sled_extensions::Db,
}

const SLED_PATH: &str = "./sled";

impl AppState {
    pub fn new() -> Self {
        let db = sled_extensions::Config::default()
            .path(SLED_PATH)
            .open()
            .expect("Failed to open sled database");

        AppState { 
            store: db,
        }
    }
}
