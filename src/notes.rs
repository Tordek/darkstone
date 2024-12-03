use std::io;

pub struct Notes {
    notes: crate::util::Query<DirectoryContents, io::ErrorKind>,
    current: Option<crate::note_editor::NoteEditor>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Create,
    Delete(String),
    SetCurrent(String),
    NoteEditorMessage(crate::note_editor::Message),
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
            iced::widget::row![
                iced::widget::mouse_area(
                    iced::widget::text(file.display_name.clone()).width(iced::Length::Fill)
                )
                .on_press(Message::SetCurrent(file.path.clone())),
                iced::widget::button(iced::widget::text("x"))
                    .on_press(Message::Delete(file.display_name.clone())),
            ]
            .padding(iced::padding::left(15)),
        );
    }
    for directory in &directory.directories {
        note_list = note_list.push(iced::widget::row![
            iced::widget::mouse_area(iced::widget::row![
                iced::widget::text(if directory.expanded { "-" } else { "+" })
                    .height(20)
                    .width(20),
                iced::widget::container(
                    iced::widget::text(directory.display_name.clone()).color([0.5, 0.5, 0.5])
                )
                .width(iced::Length::Fill)
                .padding(iced::padding::left(15))
            ])
            .on_press(Message::Expand(directory.path.clone(), !directory.expanded)),
            iced::widget::button(iced::widget::text("x"))
                .on_press(Message::Delete(directory.display_name.clone()))
        ]);
        if directory.expanded {
            note_list = note_list.push(iced::widget::stack![
                iced::widget::container(dir_tree(&directory.contents))
                    .padding(iced::padding::left(20)),
                iced::widget::container(iced::widget::vertical_rule(2))
                    .padding(iced::padding::left(10))
            ]);
        }
    }
    note_list.into()
}

impl Notes {
    pub fn new(location: String) -> (Self, iced::Task<Message>) {
        (
            Self {
                notes: crate::util::Query::Pending,
                current: None,
            },
            iced::Task::perform(load_files(location), Message::LoadFiles),
        )
    }
    pub fn view<'a>(self: &Self) -> iced::Element<'_, Message> {
        let note_list: iced::Element<'_, Message> = match &self.notes {
            crate::util::Query::Pending => iced::widget::text("Loading...").into(),
            crate::util::Query::Error(e) => iced::widget::text(format!("Error: {:?}", e)).into(),
            crate::util::Query::Loaded(directory) => iced::widget::scrollable(dir_tree(&directory))
                .height(iced::Length::Fill)
                .into(),
        };

        let sidebar = iced::widget::column![
            note_list,
            iced::widget::horizontal_rule(1),
            iced::widget::container(
                iced::widget::button(iced::widget::text("Create")).on_press(Message::Create)
            )
            .width(iced::Length::Fill)
        ]
        .spacing(4)
        .width(280)
        .padding(8);

        let main_view: iced::Element<'_, Message> = if let Some(current_note) = &self.current {
            crate::note_editor::NoteEditor::view(current_note).map(Message::NoteEditorMessage)
        } else {
            iced::widget::text("No note selected").into()
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
            Message::Delete(_name) => {
                // self.notes.retain(|note| note.path != name);
                iced::Task::none()
            }
            Message::SetCurrent(path) => {
                let (state, next_task) = crate::note_editor::NoteEditor::from_path(path);
                self.current = Some(state);
                next_task.map(Message::NoteEditorMessage)
            }
            Message::NoteEditorMessage(message) => {
                if let Some(current_note) = &mut self.current {
                    crate::note_editor::NoteEditor::update(current_note, message)
                        .map(Message::NoteEditorMessage)
                } else {
                    iced::Task::none()
                }
            }
            Message::LoadFiles(Ok(directory)) => {
                self.notes = crate::util::Query::Loaded(directory);
                iced::Task::none()
            }
            Message::LoadFiles(Err(e)) => {
                self.notes = crate::util::Query::Error(e);
                iced::Task::none()
            }
            Message::Expand(path, open) => {
                if let crate::util::Query::Loaded(directory) = &mut self.notes {
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
