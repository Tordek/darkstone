use std::path::PathBuf;

use iced::futures::TryFutureExt;

mod config;
mod note_editor;
mod notes;
mod util;

struct Darkstone {
    data: util::Query<DarkstoneData, std::io::ErrorKind>,
}

struct DarkstoneData {
    config: config::Configuration,
    notes: notes::Notes,
}

#[derive(Debug, Clone)]
enum Message {
    LoadConfig,
    LoadedConfig(Result<config::Configuration, std::io::ErrorKind>),
    SaveConfig,
    SavedConfig(Result<(), std::io::ErrorKind>),
    Notes(notes::Message),
}

impl Darkstone {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                data: util::Query::Pending,
            },
            iced::Task::done(Message::LoadConfig),
        )
    }
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::LoadConfig => iced::Task::perform(
                load_config(
                    PathBuf::from(std::env::var("HOME").unwrap()).join(".config/darkstone/config"),
                ),
                Message::LoadedConfig,
            ),
            Message::SaveConfig => match &self.data {
                util::Query::Loaded(DarkstoneData { config, notes: _ }) => {
                    // Save the config
                    iced::Task::perform(save_config(config.clone()), Message::SavedConfig)
                }
                _ => iced::Task::none(),
            },
            Message::SavedConfig(Ok(_)) => iced::Task::none(),
            Message::SavedConfig(Err(e)) => {
                self.data = util::Query::Error(e);
                iced::Task::none()
            }
            Message::LoadedConfig(Ok(config)) => {
                let (notes, notes_task) = notes::Notes::new(config.notes_path.clone());
                self.data = util::Query::Loaded(DarkstoneData {
                    config: config.clone(),
                    notes: notes,
                });
                notes_task.map(Message::Notes)
            }
            Message::LoadedConfig(Err(e)) => {
                self.data = util::Query::Error(e);
                iced::Task::none()
            }
            Message::Notes(message) => match &mut self.data {
                util::Query::Loaded(DarkstoneData {
                    ref mut notes,
                    config: _,
                }) => {
                    return notes.update(message).map(Message::Notes);
                }
                _ => iced::Task::none(),
            },
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        match self.data {
            util::Query::Pending => iced::widget::Text::new("Loading...").into(),
            util::Query::Loaded(ref data) => data.notes.view().map(Message::Notes).into(),
            util::Query::Error(ref e) => iced::widget::Text::new(format!("{:?}", e)).into(),
        }
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        match &self.data {
            util::Query::Loaded(DarkstoneData { config: _, notes }) => {
                notes.subscription().map(Message::Notes)
            }
            _ => iced::Subscription::none(),
        }
    }
}

async fn load_config(
    path: std::path::PathBuf,
) -> Result<config::Configuration, std::io::ErrorKind> {
    match crate::util::read_file(path).await {
        Ok(file) => {
            let config = config::Configuration {
                notes_path: PathBuf::from(file),
            };
            Ok(config)
        }
        Err(std::io::ErrorKind::NotFound) => {
            let config = crate::config::Configuration::default();
            save_config(config.clone()).await?;
            Ok(config)
        }
        Err(e) => Err(e),
    }
}

async fn save_config(config: config::Configuration) -> Result<(), std::io::ErrorKind> {
    let config_file = format!("notes_path: {}", config.notes_path.to_string_lossy());
    let path =
        std::path::PathBuf::from(std::env::var("HOME").unwrap()).join("/.config/darkstone/config");
    tokio::fs::write(path, config_file)
        .map_err(|e| e.kind())
        .await?;
    Ok(())
}

pub fn main() -> iced::Result {
    iced::application("Darkstone", Darkstone::update, Darkstone::view)
        .theme(|_| iced::Theme::TokyoNight)
        .subscription(Darkstone::subscription)
        .run_with(Darkstone::new)
}
