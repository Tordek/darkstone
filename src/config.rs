#[derive(Debug, Clone)]
pub struct Configuration {
    pub notes_path: std::path::PathBuf,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            notes_path: std::path::PathBuf::from(std::env::var("HOME").unwrap())
                .join("{}/.darkstone/notes"),
        }
    }
}
