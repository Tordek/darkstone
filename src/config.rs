#[derive(Debug, Clone)]
pub struct Configuration {
    pub notes_path: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            notes_path: format!("{}/.darkstone/notes", std::env::var("HOME").unwrap()),
        }
    }
}
