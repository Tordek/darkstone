mod note_editor;

pub struct Notes {
    notes: Option<DirectoryContents>,
    current: Option<note_editor::NoteEditor>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Create,
    Delete(String),
    SetCurrent(String),
    NoteEditorMessage(note_editor::Message),
    LoadFiles(Result<DirectoryContents, std::io::ErrorKind>),
    Expand(String, bool),
}

#[derive(Debug, Clone)]
pub struct DirectoryContents {
    files: Vec<File>,
    directories: Vec<Directory>,
}

#[derive(Debug, Clone)]
struct File {
    display_name: String,
    path: String,
}

#[derive(Debug, Clone)]
struct Directory {
    display_name: String,
    path: String,
    expanded: bool,
    contents: DirectoryContents,
}

fn dir_tree(directory: &DirectoryContents) -> iced::Element<'_, Message> {
    let mut note_list = iced::widget::Column::new();
    for file in &directory.files {
        note_list = note_list.push(
            iced::widget::Row::new()
                .push(
                    iced::widget::mouse_area(iced::widget::Text::new(file.display_name.clone()))
                        .on_press(Message::SetCurrent(file.path.clone())),
                )
                .push(
                    iced::widget::Button::new(iced::widget::Text::new("Delete"))
                        .on_press(Message::Delete(file.display_name.clone())),
                ),
        );
    }
    for directory in &directory.directories {
        note_list = note_list.push(iced::widget::row![
            iced::widget::mouse_area(iced::widget::row![
                iced::widget::Text::new(if directory.expanded { "-" } else { "+" }),
                iced::widget::Text::new(directory.display_name.clone()).color([0.5, 0.5, 0.5])
            ])
            .on_press(Message::Expand(directory.path.clone(), !directory.expanded)),
            iced::widget::Button::new(iced::widget::Text::new("Delete"))
                .on_press(Message::Delete(directory.display_name.clone()))
        ]);
        if directory.expanded {
            note_list = note_list.push(
                iced::widget::Row::new()
                    .push(dir_tree(&directory.contents))
                    .padding(iced::padding::left(10)),
            );
        }
    }
    note_list.into()
}

impl Notes {
    pub fn new(location: String) -> (Self, iced::Task<Message>) {
        (
            Self {
                notes: None,
                current: None,
            },
            iced::Task::perform(load_files(location), Message::LoadFiles),
        )
    }
    pub fn view<'a>(self: &Self) -> iced::Element<'_, Message> {
        let note_list: iced::Element<'_, Message> = if let Some(directory) = &self.notes {
            iced::widget::scrollable(dir_tree(&directory)).into()
        } else {
            iced::widget::Text::new("Loading...").into()
        };

        let sidebar = iced::widget::Column::new()
            .push(
                iced::widget::Button::new(iced::widget::Text::new("Create"))
                    .on_press(Message::Create),
            )
            .push(note_list);

        let main_view: iced::Element<'_, Message> = if let Some(current_note) = &self.current {
            note_editor::NoteEditor::view(current_note).map(Message::NoteEditorMessage)
        } else {
            iced::widget::Text::new("No note selected").into()
        };

        iced::widget::row![sidebar, main_view].into()
    }

    pub fn update(self: &mut Self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Create => {
                // self.notes.push(Note {
                //     path: "New Note".to_string(),
                // });
                iced::Task::none()
            }
            Message::Delete(name) => {
                // self.notes.retain(|note| note.path != name);
                iced::Task::none()
            }
            Message::SetCurrent(path) => {
                let (state, next_task) = note_editor::NoteEditor::from_path(path);
                self.current = Some(state);
                next_task.map(Message::NoteEditorMessage)
            }
            Message::NoteEditorMessage(message) => {
                if let Some(current_note) = &mut self.current {
                    note_editor::NoteEditor::update(current_note, message)
                        .map(Message::NoteEditorMessage)
                } else {
                    iced::Task::none()
                }
            }
            Message::LoadFiles(Ok(directory)) => {
                self.notes = Some(directory);
                iced::Task::none()
            }
            Message::LoadFiles(Err(e)) => {
                eprintln!("Failed to load files: {:?}", e);
                iced::Task::none()
            }
            Message::Expand(path, open) => {
                if let Some(directory) = &mut self.notes {
                    expand(directory, path, open);
                }
                iced::Task::none()
            }
        }
    }
}

fn expand(directory: &mut DirectoryContents, path: String, open: bool) {
    for dir in &mut directory.directories {
        if dir.path == path {
            dir.expanded = open;
        }
        expand(&mut dir.contents, path.clone(), open);
    }
}

async fn load_files(path: String) -> Result<DirectoryContents, std::io::ErrorKind> {
    let mut files = vec![];
    let mut directories = vec![];

    let mut entries = tokio::fs::read_dir(path).await.map_err(|e| e.kind())?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        let display_name = path.file_name().unwrap().to_string_lossy().to_string();

        if path.is_dir() {
            directories.push(Directory {
                display_name,
                expanded: true,
                path: path.to_string_lossy().to_string(),
                contents: Box::pin(load_files(
                    path.to_str().expect("Can't read pathname").to_string(),
                ))
                .await?,
            });
        } else {
            files.push(File {
                display_name,
                path: path.to_string_lossy().to_string(),
            });
        }
    }

    Ok(DirectoryContents { files, directories })
}
